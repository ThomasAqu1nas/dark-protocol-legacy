use std::{ fs, path::{ PathBuf }, sync::{ Arc, RwLock } };
use anyhow::Result;
use ark_bn254::{ Bn254, Fr };
use ark_circom::CircomCircuit;
use ark_groth16::{ Groth16, ProvingKey, VerifyingKey };
use ark_serialize::{ CanonicalDeserialize, CanonicalSerialize };
use ark_std::rand::thread_rng;

pub struct SetupParams {
    pk_path: PathBuf,
    vk_path: PathBuf,
    inner: RwLock<(ProvingKey<Bn254>, VerifyingKey<Bn254>)>,
}

impl SetupParams {
    pub fn init_or_load(
        circuit: CircomCircuit<Fr>,
        pk_path: impl Into<PathBuf>,
        vk_path: impl Into<PathBuf>
    ) -> Result<Arc<Self>> {
        let pk_path = pk_path.into();
        let vk_path = vk_path.into();

        let (pk, vk) = if pk_path.exists() && vk_path.exists() {
            let mut f = fs::File::open(&pk_path)?;
            let pk = ProvingKey::<Bn254>::deserialize_uncompressed(&mut f)?;
            let mut f = fs::File::open(&vk_path)?;
            let vk = VerifyingKey::<Bn254>::deserialize_uncompressed(&mut f)?;
            (pk, vk)
        } else {
            let rng = &mut thread_rng();
            let params =
                Groth16::<Bn254>::generate_random_parameters_with_reduction::<CircomCircuit<Fr>>(
                    circuit.clone(),
                    rng
                )?;
            let pk = params.clone();
            let vk = params.vk.clone();
            let mut f = fs::File::create(&pk_path)?;
            pk.serialize_uncompressed(&mut f)?;
            let mut f = fs::File::create(&vk_path)?;
            vk.serialize_uncompressed(&mut f)?;
            (pk, vk)
        };

        Ok(
            Arc::new(Self {
                pk_path,
                vk_path,
                inner: RwLock::new((pk, vk)),
            })
        )
    }

    /// Fast read - thread safe
    pub fn get(&self) -> (ProvingKey<Bn254>, VerifyingKey<Bn254>) {
        let guard = self.inner.read().unwrap();
        (guard.0.clone(), guard.1.clone())
    }

    /// HOT reload params - in server must be called actix non blocking wrapper
    pub fn reload(&self, circuit: &CircomCircuit<Fr>) -> Result<()> {
        let (new_pk, new_vk) = if self.pk_path.exists() && self.vk_path.exists() {
            let mut f = fs::File::open(&self.pk_path)?;
            let pk = ProvingKey::<Bn254>::deserialize_uncompressed(&mut f)?;
            let mut f = fs::File::open(&self.vk_path)?;
            let vk = VerifyingKey::<Bn254>::deserialize_uncompressed(&mut f)?;
            (pk, vk)
        } else {
            let rng = &mut thread_rng();
            let params =
                Groth16::<Bn254>::generate_random_parameters_with_reduction::<CircomCircuit<Fr>>(
                    circuit.clone(),
                    rng
                )?;
            let pk = params.clone();
            let vk = params.vk.clone();
            let mut f = fs::File::create(&self.pk_path)?;
            pk.serialize_uncompressed(&mut f)?;
            let mut f = fs::File::create(&self.vk_path)?;
            vk.serialize_uncompressed(&mut f)?;
            (pk, vk)
        };

        let mut guard = self.inner.write().unwrap();
        *guard = (new_pk, new_vk);
        Ok(())
    }
}
