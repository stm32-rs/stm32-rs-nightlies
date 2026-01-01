///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `RNGEN` reader - Random number generator enable
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - Random number generator enable
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IE` reader - Interrupt enable
pub type IE_R = crate::BitReader;
///Field `IE` writer - Interrupt enable
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CED` reader - Clock error detection
pub type CED_R = crate::BitReader;
///Field `CED` writer - Clock error detection
pub type CED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYP` reader - Bypass mode enable
pub type BYP_R = crate::BitReader;
///Field `BYP` writer - Bypass mode enable
pub type BYP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Random number generator enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Clock error detection
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Bypass mode enable
    #[inline(always)]
    pub fn byp(&self) -> BYP_R {
        BYP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rngen", &self.rngen())
            .field("ie", &self.ie())
            .field("ced", &self.ced())
            .field("byp", &self.byp())
            .finish()
    }
}
impl W {
    ///Bit 2 - Random number generator enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, CRrs> {
        RNGEN_W::new(self, 2)
    }
    ///Bit 3 - Interrupt enable
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<'_, CRrs> {
        IE_W::new(self, 3)
    }
    ///Bit 5 - Clock error detection
    #[inline(always)]
    pub fn ced(&mut self) -> CED_W<'_, CRrs> {
        CED_W::new(self, 5)
    }
    ///Bit 6 - Bypass mode enable
    #[inline(always)]
    pub fn byp(&mut self) -> BYP_W<'_, CRrs> {
        BYP_W::new(self, 6)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#RNG:CR)*/
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
