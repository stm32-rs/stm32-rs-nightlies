///Register `CRH` reader
pub type R = crate::R<CRHrs>;
///Register `CRH` writer
pub type W = crate::W<CRHrs>;
///Field `SECIE` reader - Second interrupt Enable
pub type SECIE_R = crate::BitReader;
///Field `SECIE` writer - Second interrupt Enable
pub type SECIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALRIE` reader - Alarm interrupt Enable
pub type ALRIE_R = crate::BitReader;
///Field `ALRIE` writer - Alarm interrupt Enable
pub type ALRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OWIE` reader - Overflow interrupt Enable
pub type OWIE_R = crate::BitReader;
///Field `OWIE` writer - Overflow interrupt Enable
pub type OWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Second interrupt Enable
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm interrupt Enable
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow interrupt Enable
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRH")
            .field("secie", &self.secie())
            .field("alrie", &self.alrie())
            .field("owie", &self.owie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Second interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn secie(&mut self) -> SECIE_W<CRHrs> {
        SECIE_W::new(self, 0)
    }
    ///Bit 1 - Alarm interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn alrie(&mut self) -> ALRIE_W<CRHrs> {
        ALRIE_W::new(self, 1)
    }
    ///Bit 2 - Overflow interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn owie(&mut self) -> OWIE_W<CRHrs> {
        OWIE_W::new(self, 2)
    }
}
/**RTC Control Register High

You can [`read`](crate::Reg::read) this register and get [`crh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#RTC:CRH)*/
pub struct CRHrs;
impl crate::RegisterSpec for CRHrs {
    type Ux = u32;
}
///`read()` method returns [`crh::R`](R) reader structure
impl crate::Readable for CRHrs {}
///`write(|w| ..)` method takes [`crh::W`](W) writer structure
impl crate::Writable for CRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CRH to value 0
impl crate::Resettable for CRHrs {
    const RESET_VALUE: u32 = 0;
}
