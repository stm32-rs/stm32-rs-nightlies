///Register `FMC_PATT` reader
pub type R = crate::R<FMC_PATTrs>;
///Register `FMC_PATT` writer
pub type W = crate::W<FMC_PATTrs>;
///Field `ATTSET` reader - ATTSET
pub type ATTSET_R = crate::FieldReader;
///Field `ATTSET` writer - ATTSET
pub type ATTSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTWAIT` reader - ATTWAIT
pub type ATTWAIT_R = crate::FieldReader;
///Field `ATTWAIT` writer - ATTWAIT
pub type ATTWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTHOLD` reader - ATTHOLD
pub type ATTHOLD_R = crate::FieldReader;
///Field `ATTHOLD` writer - ATTHOLD
pub type ATTHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTHIZ` reader - ATTHIZ
pub type ATTHIZ_R = crate::FieldReader;
///Field `ATTHIZ` writer - ATTHIZ
pub type ATTHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - ATTSET
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - ATTWAIT
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - ATTHOLD
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - ATTHIZ
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_PATT")
            .field("attset", &self.attset())
            .field("attwait", &self.attwait())
            .field("atthold", &self.atthold())
            .field("atthiz", &self.atthiz())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - ATTSET
    #[inline(always)]
    #[must_use]
    pub fn attset(&mut self) -> ATTSET_W<FMC_PATTrs> {
        ATTSET_W::new(self, 0)
    }
    ///Bits 8:15 - ATTWAIT
    #[inline(always)]
    #[must_use]
    pub fn attwait(&mut self) -> ATTWAIT_W<FMC_PATTrs> {
        ATTWAIT_W::new(self, 8)
    }
    ///Bits 16:23 - ATTHOLD
    #[inline(always)]
    #[must_use]
    pub fn atthold(&mut self) -> ATTHOLD_W<FMC_PATTrs> {
        ATTHOLD_W::new(self, 16)
    }
    ///Bits 24:31 - ATTHIZ
    #[inline(always)]
    #[must_use]
    pub fn atthiz(&mut self) -> ATTHIZ_W<FMC_PATTrs> {
        ATTHIZ_W::new(self, 24)
    }
}
/**The FMC_PATT read/write register contains NAND Flash memory bank timing information. It is used for 8-bit accesses to the NAND Flash attribute memory space during the last address write access when the timing differs from previous accesses (for Ready/Busy management, refer to Section25.8.5: NAND Flash prewait function).

You can [`read`](crate::Reg::read) this register and get [`fmc_patt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc_patt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:FMC_PATT)*/
pub struct FMC_PATTrs;
impl crate::RegisterSpec for FMC_PATTrs {
    type Ux = u32;
}
///`read()` method returns [`fmc_patt::R`](R) reader structure
impl crate::Readable for FMC_PATTrs {}
///`write(|w| ..)` method takes [`fmc_patt::W`](W) writer structure
impl crate::Writable for FMC_PATTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FMC_PATT to value 0x0a0a_0a0a
impl crate::Resettable for FMC_PATTrs {
    const RESET_VALUE: u32 = 0x0a0a_0a0a;
}
