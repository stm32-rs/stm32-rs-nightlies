///Register `BSR` reader
pub type R = crate::R<BSRrs>;
///Field `OVRUDR` reader - OVRUDR
pub type OVRUDR_R = crate::BitReader;
///Field `MUTEDET` reader - MUTEDET
pub type MUTEDET_R = crate::BitReader;
///Field `WCKCFG` reader - WCKCFG
pub type WCKCFG_R = crate::BitReader;
///Field `FREQ` reader - FREQ
pub type FREQ_R = crate::BitReader;
///Field `CNRDY` reader - CNRDY
pub type CNRDY_R = crate::BitReader;
///Field `AFSDET` reader - AFSDET
pub type AFSDET_R = crate::BitReader;
///Field `LFSDET` reader - LFSDET
pub type LFSDET_R = crate::BitReader;
///Field `FLVL` reader - FLVL
pub type FLVL_R = crate::FieldReader;
impl R {
    ///Bit 0 - OVRUDR
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MUTEDET
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WCKCFG
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FREQ
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CNRDY
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AFSDET
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LFSDET
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 16:18 - FLVL
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSR")
            .field("ovrudr", &self.ovrudr())
            .field("mutedet", &self.mutedet())
            .field("wckcfg", &self.wckcfg())
            .field("freq", &self.freq())
            .field("cnrdy", &self.cnrdy())
            .field("afsdet", &self.afsdet())
            .field("lfsdet", &self.lfsdet())
            .field("flvl", &self.flvl())
            .finish()
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`bsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SAI1:BSR)*/
pub struct BSRrs;
impl crate::RegisterSpec for BSRrs {
    type Ux = u32;
}
///`read()` method returns [`bsr::R`](R) reader structure
impl crate::Readable for BSRrs {}
///`reset()` method sets BSR to value 0x08
impl crate::Resettable for BSRrs {
    const RESET_VALUE: u32 = 0x08;
}
