///Register `AWD3CR` reader
pub type R = crate::R<AWD3CRrs>;
///Register `AWD3CR` writer
pub type W = crate::W<AWD3CRrs>;
/**Analog watchdog 3 channel selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH0 {
    ///0: Input channel not monitored by AWDx
    NotMonitored = 0,
    ///1: Input channel monitored by AWDx
    Monitored = 1,
}
impl From<AWD3CH0> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH0) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH0` reader - Analog watchdog 3 channel selection
pub type AWD3CH0_R = crate::BitReader<AWD3CH0>;
impl AWD3CH0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH0 {
        match self.bits {
            false => AWD3CH0::NotMonitored,
            true => AWD3CH0::Monitored,
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD3CH0::NotMonitored
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD3CH0::Monitored
    }
}
///Field `AWD3CH0` writer - Analog watchdog 3 channel selection
pub type AWD3CH0_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH0>;
impl<'a, REG> AWD3CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::NotMonitored)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::Monitored)
    }
}
///Field `AWD3CH1` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH1_R;
///Field `AWD3CH2` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH2_R;
///Field `AWD3CH3` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH3_R;
///Field `AWD3CH4` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH4_R;
///Field `AWD3CH5` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH5_R;
///Field `AWD3CH6` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH6_R;
///Field `AWD3CH7` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH7_R;
///Field `AWD3CH8` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH8_R;
///Field `AWD3CH9` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH9_R;
///Field `AWD3CH10` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH10_R;
///Field `AWD3CH11` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH11_R;
///Field `AWD3CH12` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH12_R;
///Field `AWD3CH13` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH13_R;
///Field `AWD3CH14` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH14_R;
///Field `AWD3CH15` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH15_R;
///Field `AWD3CH16` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH16_R;
///Field `AWD3CH17` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH17_R;
///Field `AWD3CH18` reader - Analog watchdog 3 channel selection
pub use AWD3CH0_R as AWD3CH18_R;
///Field `AWD3CH1` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH1_W;
///Field `AWD3CH2` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH2_W;
///Field `AWD3CH3` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH3_W;
///Field `AWD3CH4` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH4_W;
///Field `AWD3CH5` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH5_W;
///Field `AWD3CH6` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH6_W;
///Field `AWD3CH7` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH7_W;
///Field `AWD3CH8` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH8_W;
///Field `AWD3CH9` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH9_W;
///Field `AWD3CH10` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH10_W;
///Field `AWD3CH11` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH11_W;
///Field `AWD3CH12` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH12_W;
///Field `AWD3CH13` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH13_W;
///Field `AWD3CH14` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH14_W;
///Field `AWD3CH15` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH15_W;
///Field `AWD3CH16` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH16_W;
///Field `AWD3CH17` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH17_W;
///Field `AWD3CH18` writer - Analog watchdog 3 channel selection
pub use AWD3CH0_W as AWD3CH18_W;
impl R {
    ///Bit 0 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch0(&self) -> AWD3CH0_R {
        AWD3CH0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch1(&self) -> AWD3CH1_R {
        AWD3CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch2(&self) -> AWD3CH2_R {
        AWD3CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch3(&self) -> AWD3CH3_R {
        AWD3CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch4(&self) -> AWD3CH4_R {
        AWD3CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch5(&self) -> AWD3CH5_R {
        AWD3CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch6(&self) -> AWD3CH6_R {
        AWD3CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch7(&self) -> AWD3CH7_R {
        AWD3CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch8(&self) -> AWD3CH8_R {
        AWD3CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch9(&self) -> AWD3CH9_R {
        AWD3CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch10(&self) -> AWD3CH10_R {
        AWD3CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch11(&self) -> AWD3CH11_R {
        AWD3CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch12(&self) -> AWD3CH12_R {
        AWD3CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch13(&self) -> AWD3CH13_R {
        AWD3CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch14(&self) -> AWD3CH14_R {
        AWD3CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch15(&self) -> AWD3CH15_R {
        AWD3CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch16(&self) -> AWD3CH16_R {
        AWD3CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch17(&self) -> AWD3CH17_R {
        AWD3CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch18(&self) -> AWD3CH18_R {
        AWD3CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD3CR")
            .field("awd3ch0", &self.awd3ch0())
            .field("awd3ch1", &self.awd3ch1())
            .field("awd3ch2", &self.awd3ch2())
            .field("awd3ch3", &self.awd3ch3())
            .field("awd3ch4", &self.awd3ch4())
            .field("awd3ch5", &self.awd3ch5())
            .field("awd3ch6", &self.awd3ch6())
            .field("awd3ch7", &self.awd3ch7())
            .field("awd3ch8", &self.awd3ch8())
            .field("awd3ch9", &self.awd3ch9())
            .field("awd3ch10", &self.awd3ch10())
            .field("awd3ch11", &self.awd3ch11())
            .field("awd3ch12", &self.awd3ch12())
            .field("awd3ch13", &self.awd3ch13())
            .field("awd3ch14", &self.awd3ch14())
            .field("awd3ch15", &self.awd3ch15())
            .field("awd3ch16", &self.awd3ch16())
            .field("awd3ch17", &self.awd3ch17())
            .field("awd3ch18", &self.awd3ch18())
            .finish()
    }
}
impl W {
    ///Bit 0 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch0(&mut self) -> AWD3CH0_W<AWD3CRrs> {
        AWD3CH0_W::new(self, 0)
    }
    ///Bit 1 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch1(&mut self) -> AWD3CH1_W<AWD3CRrs> {
        AWD3CH1_W::new(self, 1)
    }
    ///Bit 2 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch2(&mut self) -> AWD3CH2_W<AWD3CRrs> {
        AWD3CH2_W::new(self, 2)
    }
    ///Bit 3 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch3(&mut self) -> AWD3CH3_W<AWD3CRrs> {
        AWD3CH3_W::new(self, 3)
    }
    ///Bit 4 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch4(&mut self) -> AWD3CH4_W<AWD3CRrs> {
        AWD3CH4_W::new(self, 4)
    }
    ///Bit 5 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch5(&mut self) -> AWD3CH5_W<AWD3CRrs> {
        AWD3CH5_W::new(self, 5)
    }
    ///Bit 6 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch6(&mut self) -> AWD3CH6_W<AWD3CRrs> {
        AWD3CH6_W::new(self, 6)
    }
    ///Bit 7 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch7(&mut self) -> AWD3CH7_W<AWD3CRrs> {
        AWD3CH7_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch8(&mut self) -> AWD3CH8_W<AWD3CRrs> {
        AWD3CH8_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch9(&mut self) -> AWD3CH9_W<AWD3CRrs> {
        AWD3CH9_W::new(self, 9)
    }
    ///Bit 10 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch10(&mut self) -> AWD3CH10_W<AWD3CRrs> {
        AWD3CH10_W::new(self, 10)
    }
    ///Bit 11 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch11(&mut self) -> AWD3CH11_W<AWD3CRrs> {
        AWD3CH11_W::new(self, 11)
    }
    ///Bit 12 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch12(&mut self) -> AWD3CH12_W<AWD3CRrs> {
        AWD3CH12_W::new(self, 12)
    }
    ///Bit 13 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch13(&mut self) -> AWD3CH13_W<AWD3CRrs> {
        AWD3CH13_W::new(self, 13)
    }
    ///Bit 14 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch14(&mut self) -> AWD3CH14_W<AWD3CRrs> {
        AWD3CH14_W::new(self, 14)
    }
    ///Bit 15 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch15(&mut self) -> AWD3CH15_W<AWD3CRrs> {
        AWD3CH15_W::new(self, 15)
    }
    ///Bit 16 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch16(&mut self) -> AWD3CH16_W<AWD3CRrs> {
        AWD3CH16_W::new(self, 16)
    }
    ///Bit 17 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch17(&mut self) -> AWD3CH17_W<AWD3CRrs> {
        AWD3CH17_W::new(self, 17)
    }
    ///Bit 18 - Analog watchdog 3 channel selection
    #[inline(always)]
    pub fn awd3ch18(&mut self) -> AWD3CH18_W<AWD3CRrs> {
        AWD3CH18_W::new(self, 18)
    }
}
/**Analog Watchdog 3 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#ADC3:AWD3CR)*/
pub struct AWD3CRrs;
impl crate::RegisterSpec for AWD3CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd3cr::R`](R) reader structure
impl crate::Readable for AWD3CRrs {}
///`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure
impl crate::Writable for AWD3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AWD3CR to value 0
impl crate::Resettable for AWD3CRrs {
    const RESET_VALUE: u32 = 0;
}
