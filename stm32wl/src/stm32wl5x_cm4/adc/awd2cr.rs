#[doc = "Register `AWD2CR` reader"]
pub type R = crate::R<AWD2CRrs>;
#[doc = "Register `AWD2CR` writer"]
pub type W = crate::W<AWD2CRrs>;
#[doc = "AWD2CH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH0 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD2"]
    NotMonitored = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD2"]
    Monitored = 1,
}
impl From<AWD2CH0> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH0` reader - AWD2CH"]
pub type AWD2CH0_R = crate::BitReader<AWD2CH0>;
impl AWD2CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH0 {
        match self.bits {
            false => AWD2CH0::NotMonitored,
            true => AWD2CH0::Monitored,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD2CH0::NotMonitored
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD2CH0::Monitored
    }
}
#[doc = "Field `AWD2CH0` writer - AWD2CH"]
pub type AWD2CH0_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH0>;
impl<'a, REG> AWD2CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD2"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH0::NotMonitored)
    }
    #[doc = "ADC analog channel-x is monitored by AWD2"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH0::Monitored)
    }
}
#[doc = "Field `AWD2CH1` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH1_R;
#[doc = "Field `AWD2CH2` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH2_R;
#[doc = "Field `AWD2CH3` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH3_R;
#[doc = "Field `AWD2CH4` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH4_R;
#[doc = "Field `AWD2CH5` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH5_R;
#[doc = "Field `AWD2CH6` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH6_R;
#[doc = "Field `AWD2CH7` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH7_R;
#[doc = "Field `AWD2CH8` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH8_R;
#[doc = "Field `AWD2CH9` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH9_R;
#[doc = "Field `AWD2CH10` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH10_R;
#[doc = "Field `AWD2CH11` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH11_R;
#[doc = "Field `AWD2CH12` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH12_R;
#[doc = "Field `AWD2CH13` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH13_R;
#[doc = "Field `AWD2CH14` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH14_R;
#[doc = "Field `AWD2CH15` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH15_R;
#[doc = "Field `AWD2CH16` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH16_R;
#[doc = "Field `AWD2CH17` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH17_R;
#[doc = "Field `AWD2CH1` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH1_W;
#[doc = "Field `AWD2CH2` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH2_W;
#[doc = "Field `AWD2CH3` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH3_W;
#[doc = "Field `AWD2CH4` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH4_W;
#[doc = "Field `AWD2CH5` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH5_W;
#[doc = "Field `AWD2CH6` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH6_W;
#[doc = "Field `AWD2CH7` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH7_W;
#[doc = "Field `AWD2CH8` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH8_W;
#[doc = "Field `AWD2CH9` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH9_W;
#[doc = "Field `AWD2CH10` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH10_W;
#[doc = "Field `AWD2CH11` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH11_W;
#[doc = "Field `AWD2CH12` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH12_W;
#[doc = "Field `AWD2CH13` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH13_W;
#[doc = "Field `AWD2CH14` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH14_W;
#[doc = "Field `AWD2CH15` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH15_W;
#[doc = "Field `AWD2CH16` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH16_W;
#[doc = "Field `AWD2CH17` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH17_W;
impl R {
    #[doc = "Bit 0 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch0(&self) -> AWD2CH0_R {
        AWD2CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch1(&self) -> AWD2CH1_R {
        AWD2CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch2(&self) -> AWD2CH2_R {
        AWD2CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch3(&self) -> AWD2CH3_R {
        AWD2CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch4(&self) -> AWD2CH4_R {
        AWD2CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch5(&self) -> AWD2CH5_R {
        AWD2CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch6(&self) -> AWD2CH6_R {
        AWD2CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch7(&self) -> AWD2CH7_R {
        AWD2CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch8(&self) -> AWD2CH8_R {
        AWD2CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch9(&self) -> AWD2CH9_R {
        AWD2CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch10(&self) -> AWD2CH10_R {
        AWD2CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch11(&self) -> AWD2CH11_R {
        AWD2CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch12(&self) -> AWD2CH12_R {
        AWD2CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch13(&self) -> AWD2CH13_R {
        AWD2CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch14(&self) -> AWD2CH14_R {
        AWD2CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch15(&self) -> AWD2CH15_R {
        AWD2CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch16(&self) -> AWD2CH16_R {
        AWD2CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch17(&self) -> AWD2CH17_R {
        AWD2CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch0(&mut self) -> AWD2CH0_W<AWD2CRrs> {
        AWD2CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch1(&mut self) -> AWD2CH1_W<AWD2CRrs> {
        AWD2CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch2(&mut self) -> AWD2CH2_W<AWD2CRrs> {
        AWD2CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch3(&mut self) -> AWD2CH3_W<AWD2CRrs> {
        AWD2CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch4(&mut self) -> AWD2CH4_W<AWD2CRrs> {
        AWD2CH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch5(&mut self) -> AWD2CH5_W<AWD2CRrs> {
        AWD2CH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch6(&mut self) -> AWD2CH6_W<AWD2CRrs> {
        AWD2CH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch7(&mut self) -> AWD2CH7_W<AWD2CRrs> {
        AWD2CH7_W::new(self, 7)
    }
    #[doc = "Bit 8 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch8(&mut self) -> AWD2CH8_W<AWD2CRrs> {
        AWD2CH8_W::new(self, 8)
    }
    #[doc = "Bit 9 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch9(&mut self) -> AWD2CH9_W<AWD2CRrs> {
        AWD2CH9_W::new(self, 9)
    }
    #[doc = "Bit 10 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch10(&mut self) -> AWD2CH10_W<AWD2CRrs> {
        AWD2CH10_W::new(self, 10)
    }
    #[doc = "Bit 11 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch11(&mut self) -> AWD2CH11_W<AWD2CRrs> {
        AWD2CH11_W::new(self, 11)
    }
    #[doc = "Bit 12 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch12(&mut self) -> AWD2CH12_W<AWD2CRrs> {
        AWD2CH12_W::new(self, 12)
    }
    #[doc = "Bit 13 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch13(&mut self) -> AWD2CH13_W<AWD2CRrs> {
        AWD2CH13_W::new(self, 13)
    }
    #[doc = "Bit 14 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch14(&mut self) -> AWD2CH14_W<AWD2CRrs> {
        AWD2CH14_W::new(self, 14)
    }
    #[doc = "Bit 15 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch15(&mut self) -> AWD2CH15_W<AWD2CRrs> {
        AWD2CH15_W::new(self, 15)
    }
    #[doc = "Bit 16 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch16(&mut self) -> AWD2CH16_W<AWD2CRrs> {
        AWD2CH16_W::new(self, 16)
    }
    #[doc = "Bit 17 - AWD2CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd2ch17(&mut self) -> AWD2CH17_W<AWD2CRrs> {
        AWD2CH17_W::new(self, 17)
    }
}
#[doc = "ADC Analog Watchdog 2 Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWD2CRrs;
impl crate::RegisterSpec for AWD2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd2cr::R`](R) reader structure"]
impl crate::Readable for AWD2CRrs {}
#[doc = "`write(|w| ..)` method takes [`awd2cr::W`](W) writer structure"]
impl crate::Writable for AWD2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWD2CR to value 0"]
impl crate::Resettable for AWD2CRrs {
    const RESET_VALUE: u32 = 0;
}
