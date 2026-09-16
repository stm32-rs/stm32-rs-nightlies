///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `INIT` reader - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0.
pub type INIT_R = crate::BitReader;
///Field `INIT` writer - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0.
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAE` reader - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit.
pub type DMAE_R = crate::BitReader;
///Field `DMAE` writer - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit.
pub type DMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATATYPE` reader - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:
pub type DATATYPE_R = crate::FieldReader;
///Field `DATATYPE` writer - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MODE` reader - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.
pub type MODE_R = crate::BitReader;
///Field `MODE` writer - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBW` reader - Number of words already pushed Refer to NBWP\[3:0\] bitfield of HASH_SR for a description of NBW\[3:0\] bitfield. This bit is read-only.
pub type NBW_R = crate::FieldReader;
///Field `DINNE` reader - DIN not empty Refer to DINNE bit of HASH_SR for a description of DINNE bit. This bit is read-only.
pub type DINNE_R = crate::BitReader;
///Field `MDMAT` reader - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed.
pub type MDMAT_R = crate::BitReader;
///Field `MDMAT` writer - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed.
pub type MDMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LKEY` reader - Long key selection The application must set this bit if the HMAC key is greater than the block size corresponding to the hash algorithm (see algorithms for details). For example the block size is 64 bytes for SHA2-256. This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect.
pub type LKEY_R = crate::BitReader;
///Field `LKEY` writer - Long key selection The application must set this bit if the HMAC key is greater than the block size corresponding to the hash algorithm (see algorithms for details). For example the block size is 64 bytes for SHA2-256. This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect.
pub type LKEY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALGO` reader - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11.
pub type ALGO_R = crate::FieldReader;
///Field `ALGO` writer - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11.
pub type ALGO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 2 - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0.
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit.
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:11 - Number of words already pushed Refer to NBWP\[3:0\] bitfield of HASH_SR for a description of NBW\[3:0\] bitfield. This bit is read-only.
    #[inline(always)]
    pub fn nbw(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - DIN not empty Refer to DINNE bit of HASH_SR for a description of DINNE bit. This bit is read-only.
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed.
    #[inline(always)]
    pub fn mdmat(&self) -> MDMAT_R {
        MDMAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Long key selection The application must set this bit if the HMAC key is greater than the block size corresponding to the hash algorithm (see algorithms for details). For example the block size is 64 bytes for SHA2-256. This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect.
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:20 - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11.
    #[inline(always)]
    pub fn algo(&self) -> ALGO_R {
        ALGO_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("init", &self.init())
            .field("dmae", &self.dmae())
            .field("datatype", &self.datatype())
            .field("mode", &self.mode())
            .field("nbw", &self.nbw())
            .field("dinne", &self.dinne())
            .field("mdmat", &self.mdmat())
            .field("lkey", &self.lkey())
            .field("algo", &self.algo())
            .finish()
    }
}
impl W {
    ///Bit 2 - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0.
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<'_, CRrs> {
        INIT_W::new(self, 2)
    }
    ///Bit 3 - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit.
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W<'_, CRrs> {
        DMAE_W::new(self, 3)
    }
    ///Bits 4:5 - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W<'_, CRrs> {
        DATATYPE_W::new(self, 4)
    }
    ///Bit 6 - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, CRrs> {
        MODE_W::new(self, 6)
    }
    ///Bit 13 - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed.
    #[inline(always)]
    pub fn mdmat(&mut self) -> MDMAT_W<'_, CRrs> {
        MDMAT_W::new(self, 13)
    }
    ///Bit 16 - Long key selection The application must set this bit if the HMAC key is greater than the block size corresponding to the hash algorithm (see algorithms for details). For example the block size is 64 bytes for SHA2-256. This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect.
    #[inline(always)]
    pub fn lkey(&mut self) -> LKEY_W<'_, CRrs> {
        LKEY_W::new(self, 16)
    }
    ///Bits 17:20 - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11.
    #[inline(always)]
    pub fn algo(&mut self) -> ALGO_W<'_, CRrs> {
        ALGO_W::new(self, 17)
    }
}
/**HASH control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#HASH:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
