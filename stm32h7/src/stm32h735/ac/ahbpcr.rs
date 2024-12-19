///Register `AHBPCR` reader
pub type R = crate::R<AHBPCRrs>;
///Register `AHBPCR` writer
pub type W = crate::W<AHBPCRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SZ` reader - SZ
pub type SZ_R = crate::FieldReader;
///Field `SZ` writer - SZ
pub type SZ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - SZ
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBPCR")
            .field("en", &self.en())
            .field("sz", &self.sz())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<AHBPCRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 1:3 - SZ
    #[inline(always)]
    pub fn sz(&mut self) -> SZ_W<AHBPCRrs> {
        SZ_W::new(self, 1)
    }
}
/**AHBP Control register

You can [`read`](crate::Reg::read) this register and get [`ahbpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H73x.html#AC:AHBPCR)*/
pub struct AHBPCRrs;
impl crate::RegisterSpec for AHBPCRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbpcr::R`](R) reader structure
impl crate::Readable for AHBPCRrs {}
///`write(|w| ..)` method takes [`ahbpcr::W`](W) writer structure
impl crate::Writable for AHBPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHBPCR to value 0
impl crate::Resettable for AHBPCRrs {
    const RESET_VALUE: u32 = 0;
}
