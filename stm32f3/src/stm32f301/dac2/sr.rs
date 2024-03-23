#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "DAC channel1 DMA underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDR1 {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel X"]
    NoUnderrun = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel X"]
    Underrun = 1,
}
impl From<DMAUDR1> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag"]
pub type DMAUDR1_R = crate::BitReader<DMAUDR1>;
impl DMAUDR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDR1 {
        match self.bits {
            false => DMAUDR1::NoUnderrun,
            true => DMAUDR1::Underrun,
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == DMAUDR1::NoUnderrun
    }
    #[doc = "DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == DMAUDR1::Underrun
    }
}
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag"]
pub type DMAUDR1_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDR1>;
impl<'a, REG> DMAUDR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1::NoUnderrun)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1::Underrun)
    }
}
impl R {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<SRrs> {
        DMAUDR1_W::new(self, 13)
    }
}
#[doc = "DAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
