///Register `POSTP_SR` reader
pub type R = crate::R<POSTP_SRrs>;
///Field `AES_INIT` reader - AES Post processing has been fully initialized (key and state) and is ready for generating 128-bit random words.
pub type AES_INIT_R = crate::BitReader;
///Field `AES_KEY_LD` reader - AES random key has been generated and loaded in AES key register.
pub type AES_KEY_LD_R = crate::BitReader;
///Field `AES_BUSY` reader - AES core is busy, generating a random value.
pub type AES_BUSY_R = crate::BitReader;
///Field `AES_HEALTH_DONE` reader - AES-CMAC health test is completed
pub type AES_HEALTH_DONE_R = crate::BitReader;
///Field `AES_K12_ERROR` reader - Health test error on AES-CMAC sub-keys generation
pub type AES_K12_ERROR_R = crate::BitReader;
///Field `AES_DOUT_ERROR` reader - Health test error on AES-CMAC output generation
pub type AES_DOUT_ERROR_R = crate::BitReader;
impl R {
    ///Bit 1 - AES Post processing has been fully initialized (key and state) and is ready for generating 128-bit random words.
    #[inline(always)]
    pub fn aes_init(&self) -> AES_INIT_R {
        AES_INIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AES random key has been generated and loaded in AES key register.
    #[inline(always)]
    pub fn aes_key_ld(&self) -> AES_KEY_LD_R {
        AES_KEY_LD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AES core is busy, generating a random value.
    #[inline(always)]
    pub fn aes_busy(&self) -> AES_BUSY_R {
        AES_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AES-CMAC health test is completed
    #[inline(always)]
    pub fn aes_health_done(&self) -> AES_HEALTH_DONE_R {
        AES_HEALTH_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Health test error on AES-CMAC sub-keys generation
    #[inline(always)]
    pub fn aes_k12_error(&self) -> AES_K12_ERROR_R {
        AES_K12_ERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Health test error on AES-CMAC output generation
    #[inline(always)]
    pub fn aes_dout_error(&self) -> AES_DOUT_ERROR_R {
        AES_DOUT_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POSTP_SR")
            .field("aes_init", &self.aes_init())
            .field("aes_key_ld", &self.aes_key_ld())
            .field("aes_busy", &self.aes_busy())
            .field("aes_health_done", &self.aes_health_done())
            .field("aes_k12_error", &self.aes_k12_error())
            .field("aes_dout_error", &self.aes_dout_error())
            .finish()
    }
}
/**TRNG_POSTP_SR register

You can [`read`](crate::Reg::read) this register and get [`postp_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:POSTP_SR)*/
pub struct POSTP_SRrs;
impl crate::RegisterSpec for POSTP_SRrs {
    type Ux = u32;
}
///`read()` method returns [`postp_sr::R`](R) reader structure
impl crate::Readable for POSTP_SRrs {}
///`reset()` method sets POSTP_SR to value 0
impl crate::Resettable for POSTP_SRrs {}
