///Register `C4ESR` reader
pub type R = crate::R<C4ESRrs>;
///Field `TEA` reader - TEA
pub type TEA_R = crate::FieldReader;
///Field `TED` reader - TED
pub type TED_R = crate::BitReader;
///Field `TELD` reader - TELD
pub type TELD_R = crate::BitReader;
///Field `TEMD` reader - TEMD
pub type TEMD_R = crate::BitReader;
///Field `ASE` reader - ASE
pub type ASE_R = crate::BitReader;
///Field `BSE` reader - BSE
pub type BSE_R = crate::BitReader;
impl R {
    ///Bits 0:6 - TEA
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - TED
    #[inline(always)]
    pub fn ted(&self) -> TED_R {
        TED_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TELD
    #[inline(always)]
    pub fn teld(&self) -> TELD_R {
        TELD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TEMD
    #[inline(always)]
    pub fn temd(&self) -> TEMD_R {
        TEMD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ASE
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BSE
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C4ESR")
            .field("tea", &self.tea())
            .field("ted", &self.ted())
            .field("teld", &self.teld())
            .field("temd", &self.temd())
            .field("ase", &self.ase())
            .field("bse", &self.bse())
            .finish()
    }
}
/**MDMA channel 4 error status register

You can [`read`](crate::Reg::read) this register and get [`c4esr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:C4ESR)*/
pub struct C4ESRrs;
impl crate::RegisterSpec for C4ESRrs {
    type Ux = u32;
}
///`read()` method returns [`c4esr::R`](R) reader structure
impl crate::Readable for C4ESRrs {}
///`reset()` method sets C4ESR to value 0
impl crate::Resettable for C4ESRrs {}
