#[doc = "Register `AWD3CR` reader"]
pub type R = crate::R<AWD3CRrs>;
#[doc = "Register `AWD3CR` writer"]
pub type W = crate::W<AWD3CRrs>;
#[doc = "AWD3CH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH0 {
    #[doc = "0: ADC analog channel-x is not monitored by AWD3"]
    NotMonitored = 0,
    #[doc = "1: ADC analog channel-x is monitored by AWD3"]
    Monitored = 1,
}
impl From<AWD3CH0> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH0` reader - AWD3CH"]
pub type AWD3CH0_R = crate::BitReader<AWD3CH0>;
impl AWD3CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH0 {
        match self.bits {
            false => AWD3CH0::NotMonitored,
            true => AWD3CH0::Monitored,
        }
    }
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD3CH0::NotMonitored
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD3CH0::Monitored
    }
}
#[doc = "Field `AWD3CH0` writer - AWD3CH"]
pub type AWD3CH0_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH0>;
impl<'a, REG> AWD3CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog channel-x is not monitored by AWD3"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::NotMonitored)
    }
    #[doc = "ADC analog channel-x is monitored by AWD3"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::Monitored)
    }
}
#[doc = "Field `AWD3CH1` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH1_R;
#[doc = "Field `AWD3CH2` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH2_R;
#[doc = "Field `AWD3CH3` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH3_R;
#[doc = "Field `AWD3CH4` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH4_R;
#[doc = "Field `AWD3CH5` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH5_R;
#[doc = "Field `AWD3CH6` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH6_R;
#[doc = "Field `AWD3CH7` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH7_R;
#[doc = "Field `AWD3CH8` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH8_R;
#[doc = "Field `AWD3CH9` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH9_R;
#[doc = "Field `AWD3CH10` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH10_R;
#[doc = "Field `AWD3CH11` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH11_R;
#[doc = "Field `AWD3CH12` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH12_R;
#[doc = "Field `AWD3CH13` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH13_R;
#[doc = "Field `AWD3CH14` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH14_R;
#[doc = "Field `AWD3CH15` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH15_R;
#[doc = "Field `AWD3CH16` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH16_R;
#[doc = "Field `AWD3CH17` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH17_R;
#[doc = "Field `AWD3CH1` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH1_W;
#[doc = "Field `AWD3CH2` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH2_W;
#[doc = "Field `AWD3CH3` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH3_W;
#[doc = "Field `AWD3CH4` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH4_W;
#[doc = "Field `AWD3CH5` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH5_W;
#[doc = "Field `AWD3CH6` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH6_W;
#[doc = "Field `AWD3CH7` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH7_W;
#[doc = "Field `AWD3CH8` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH8_W;
#[doc = "Field `AWD3CH9` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH9_W;
#[doc = "Field `AWD3CH10` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH10_W;
#[doc = "Field `AWD3CH11` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH11_W;
#[doc = "Field `AWD3CH12` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH12_W;
#[doc = "Field `AWD3CH13` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH13_W;
#[doc = "Field `AWD3CH14` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH14_W;
#[doc = "Field `AWD3CH15` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH15_W;
#[doc = "Field `AWD3CH16` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH16_W;
#[doc = "Field `AWD3CH17` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH17_W;
impl R {
    #[doc = "Bit 0 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch0(&self) -> AWD3CH0_R {
        AWD3CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch1(&self) -> AWD3CH1_R {
        AWD3CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch2(&self) -> AWD3CH2_R {
        AWD3CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch3(&self) -> AWD3CH3_R {
        AWD3CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch4(&self) -> AWD3CH4_R {
        AWD3CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch5(&self) -> AWD3CH5_R {
        AWD3CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch6(&self) -> AWD3CH6_R {
        AWD3CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch7(&self) -> AWD3CH7_R {
        AWD3CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch8(&self) -> AWD3CH8_R {
        AWD3CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch9(&self) -> AWD3CH9_R {
        AWD3CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch10(&self) -> AWD3CH10_R {
        AWD3CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch11(&self) -> AWD3CH11_R {
        AWD3CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch12(&self) -> AWD3CH12_R {
        AWD3CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch13(&self) -> AWD3CH13_R {
        AWD3CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch14(&self) -> AWD3CH14_R {
        AWD3CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch15(&self) -> AWD3CH15_R {
        AWD3CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch16(&self) -> AWD3CH16_R {
        AWD3CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch17(&self) -> AWD3CH17_R {
        AWD3CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch0(&mut self) -> AWD3CH0_W<AWD3CRrs> {
        AWD3CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch1(&mut self) -> AWD3CH1_W<AWD3CRrs> {
        AWD3CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch2(&mut self) -> AWD3CH2_W<AWD3CRrs> {
        AWD3CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch3(&mut self) -> AWD3CH3_W<AWD3CRrs> {
        AWD3CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch4(&mut self) -> AWD3CH4_W<AWD3CRrs> {
        AWD3CH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch5(&mut self) -> AWD3CH5_W<AWD3CRrs> {
        AWD3CH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch6(&mut self) -> AWD3CH6_W<AWD3CRrs> {
        AWD3CH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch7(&mut self) -> AWD3CH7_W<AWD3CRrs> {
        AWD3CH7_W::new(self, 7)
    }
    #[doc = "Bit 8 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch8(&mut self) -> AWD3CH8_W<AWD3CRrs> {
        AWD3CH8_W::new(self, 8)
    }
    #[doc = "Bit 9 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch9(&mut self) -> AWD3CH9_W<AWD3CRrs> {
        AWD3CH9_W::new(self, 9)
    }
    #[doc = "Bit 10 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch10(&mut self) -> AWD3CH10_W<AWD3CRrs> {
        AWD3CH10_W::new(self, 10)
    }
    #[doc = "Bit 11 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch11(&mut self) -> AWD3CH11_W<AWD3CRrs> {
        AWD3CH11_W::new(self, 11)
    }
    #[doc = "Bit 12 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch12(&mut self) -> AWD3CH12_W<AWD3CRrs> {
        AWD3CH12_W::new(self, 12)
    }
    #[doc = "Bit 13 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch13(&mut self) -> AWD3CH13_W<AWD3CRrs> {
        AWD3CH13_W::new(self, 13)
    }
    #[doc = "Bit 14 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch14(&mut self) -> AWD3CH14_W<AWD3CRrs> {
        AWD3CH14_W::new(self, 14)
    }
    #[doc = "Bit 15 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch15(&mut self) -> AWD3CH15_W<AWD3CRrs> {
        AWD3CH15_W::new(self, 15)
    }
    #[doc = "Bit 16 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch16(&mut self) -> AWD3CH16_W<AWD3CRrs> {
        AWD3CH16_W::new(self, 16)
    }
    #[doc = "Bit 17 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch17(&mut self) -> AWD3CH17_W<AWD3CRrs> {
        AWD3CH17_W::new(self, 17)
    }
}
#[doc = "ADC Analog Watchdog 3 Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWD3CRrs;
impl crate::RegisterSpec for AWD3CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd3cr::R`](R) reader structure"]
impl crate::Readable for AWD3CRrs {}
#[doc = "`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure"]
impl crate::Writable for AWD3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWD3CR to value 0"]
impl crate::Resettable for AWD3CRrs {
    const RESET_VALUE: u32 = 0;
}
