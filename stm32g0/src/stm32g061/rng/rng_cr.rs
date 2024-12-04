///Register `RNG_CR` reader
pub type R = crate::R<RNG_CRrs>;
///Register `RNG_CR` writer
pub type W = crate::W<RNG_CRrs>;
///Field `RNGEN` reader - True random number generator enable
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - True random number generator enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE` reader - Interrupt Enable
pub type IE_R = crate::BitReader;
///Field `IE` writer - Interrupt Enable
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CED` reader - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled.
pub type CED_R = crate::BitReader;
///Field `CED` writer - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled.
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled.
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_CR")
            .field("rngen", &self.rngen())
            .field("ie", &self.ie())
            .field("ced", &self.ced())
            .finish()
    }
}
impl W {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<RNG_CRrs> {
        RNGEN_W::new(self, 2)
    }
    ///Bit 3 - Interrupt Enable
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<RNG_CRrs> {
        IE_W::new(self, 3)
    }
    ///Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled.
    #[inline(always)]
    pub fn ced(&mut self) -> CED_W<RNG_CRrs> {
        CED_W::new(self, 5)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`rng_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#RNG:RNG_CR)*/
pub struct RNG_CRrs;
impl crate::RegisterSpec for RNG_CRrs {
    type Ux = u32;
}
///`read()` method returns [`rng_cr::R`](R) reader structure
impl crate::Readable for RNG_CRrs {}
///`write(|w| ..)` method takes [`rng_cr::W`](W) writer structure
impl crate::Writable for RNG_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RNG_CR to value 0
impl crate::Resettable for RNG_CRrs {
    const RESET_VALUE: u32 = 0;
}
