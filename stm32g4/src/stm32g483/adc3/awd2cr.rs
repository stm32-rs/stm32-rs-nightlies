///Register `AWD2CR` reader
pub type R = crate::R<AWD2CRrs>;
///Register `AWD2CR` writer
pub type W = crate::W<AWD2CRrs>;
/**Analog watchdog 2 channel selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2CH0 {
    ///0: Input channel not monitored by AWDx
    NotMonitored = 0,
    ///1: Input channel monitored by AWDx
    Monitored = 1,
}
impl From<AWD2CH0> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH0) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD2CH0` reader - Analog watchdog 2 channel selection
pub type AWD2CH0_R = crate::BitReader<AWD2CH0>;
impl AWD2CH0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD2CH0 {
        match self.bits {
            false => AWD2CH0::NotMonitored,
            true => AWD2CH0::Monitored,
        }
    }
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD2CH0::NotMonitored
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD2CH0::Monitored
    }
}
///Field `AWD2CH0` writer - Analog watchdog 2 channel selection
pub type AWD2CH0_W<'a, REG> = crate::BitWriter<'a, REG, AWD2CH0>;
impl<'a, REG> AWD2CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Input channel not monitored by AWDx
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH0::NotMonitored)
    }
    ///Input channel monitored by AWDx
    #[inline(always)]
    pub fn monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2CH0::Monitored)
    }
}
///Field `AWD2CH1` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH1_R;
///Field `AWD2CH2` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH2_R;
///Field `AWD2CH3` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH3_R;
///Field `AWD2CH4` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH4_R;
///Field `AWD2CH5` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH5_R;
///Field `AWD2CH6` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH6_R;
///Field `AWD2CH7` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH7_R;
///Field `AWD2CH8` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH8_R;
///Field `AWD2CH9` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH9_R;
///Field `AWD2CH10` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH10_R;
///Field `AWD2CH11` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH11_R;
///Field `AWD2CH12` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH12_R;
///Field `AWD2CH13` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH13_R;
///Field `AWD2CH14` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH14_R;
///Field `AWD2CH15` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH15_R;
///Field `AWD2CH16` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH16_R;
///Field `AWD2CH17` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH17_R;
///Field `AWD2CH18` reader - Analog watchdog 2 channel selection
pub use AWD2CH0_R as AWD2CH18_R;
///Field `AWD2CH1` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH1_W;
///Field `AWD2CH2` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH2_W;
///Field `AWD2CH3` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH3_W;
///Field `AWD2CH4` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH4_W;
///Field `AWD2CH5` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH5_W;
///Field `AWD2CH6` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH6_W;
///Field `AWD2CH7` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH7_W;
///Field `AWD2CH8` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH8_W;
///Field `AWD2CH9` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH9_W;
///Field `AWD2CH10` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH10_W;
///Field `AWD2CH11` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH11_W;
///Field `AWD2CH12` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH12_W;
///Field `AWD2CH13` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH13_W;
///Field `AWD2CH14` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH14_W;
///Field `AWD2CH15` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH15_W;
///Field `AWD2CH16` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH16_W;
///Field `AWD2CH17` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH17_W;
///Field `AWD2CH18` writer - Analog watchdog 2 channel selection
pub use AWD2CH0_W as AWD2CH18_W;
impl R {
    ///Bit 0 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch0(&self) -> AWD2CH0_R {
        AWD2CH0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch1(&self) -> AWD2CH1_R {
        AWD2CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch2(&self) -> AWD2CH2_R {
        AWD2CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch3(&self) -> AWD2CH3_R {
        AWD2CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch4(&self) -> AWD2CH4_R {
        AWD2CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch5(&self) -> AWD2CH5_R {
        AWD2CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch6(&self) -> AWD2CH6_R {
        AWD2CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch7(&self) -> AWD2CH7_R {
        AWD2CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch8(&self) -> AWD2CH8_R {
        AWD2CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch9(&self) -> AWD2CH9_R {
        AWD2CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch10(&self) -> AWD2CH10_R {
        AWD2CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch11(&self) -> AWD2CH11_R {
        AWD2CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch12(&self) -> AWD2CH12_R {
        AWD2CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch13(&self) -> AWD2CH13_R {
        AWD2CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch14(&self) -> AWD2CH14_R {
        AWD2CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch15(&self) -> AWD2CH15_R {
        AWD2CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch16(&self) -> AWD2CH16_R {
        AWD2CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch17(&self) -> AWD2CH17_R {
        AWD2CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch18(&self) -> AWD2CH18_R {
        AWD2CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD2CR")
            .field("awd2ch0", &self.awd2ch0())
            .field("awd2ch1", &self.awd2ch1())
            .field("awd2ch2", &self.awd2ch2())
            .field("awd2ch3", &self.awd2ch3())
            .field("awd2ch4", &self.awd2ch4())
            .field("awd2ch5", &self.awd2ch5())
            .field("awd2ch6", &self.awd2ch6())
            .field("awd2ch7", &self.awd2ch7())
            .field("awd2ch8", &self.awd2ch8())
            .field("awd2ch9", &self.awd2ch9())
            .field("awd2ch10", &self.awd2ch10())
            .field("awd2ch11", &self.awd2ch11())
            .field("awd2ch12", &self.awd2ch12())
            .field("awd2ch13", &self.awd2ch13())
            .field("awd2ch14", &self.awd2ch14())
            .field("awd2ch15", &self.awd2ch15())
            .field("awd2ch16", &self.awd2ch16())
            .field("awd2ch17", &self.awd2ch17())
            .field("awd2ch18", &self.awd2ch18())
            .finish()
    }
}
impl W {
    ///Bit 0 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch0(&mut self) -> AWD2CH0_W<AWD2CRrs> {
        AWD2CH0_W::new(self, 0)
    }
    ///Bit 1 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch1(&mut self) -> AWD2CH1_W<AWD2CRrs> {
        AWD2CH1_W::new(self, 1)
    }
    ///Bit 2 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch2(&mut self) -> AWD2CH2_W<AWD2CRrs> {
        AWD2CH2_W::new(self, 2)
    }
    ///Bit 3 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch3(&mut self) -> AWD2CH3_W<AWD2CRrs> {
        AWD2CH3_W::new(self, 3)
    }
    ///Bit 4 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch4(&mut self) -> AWD2CH4_W<AWD2CRrs> {
        AWD2CH4_W::new(self, 4)
    }
    ///Bit 5 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch5(&mut self) -> AWD2CH5_W<AWD2CRrs> {
        AWD2CH5_W::new(self, 5)
    }
    ///Bit 6 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch6(&mut self) -> AWD2CH6_W<AWD2CRrs> {
        AWD2CH6_W::new(self, 6)
    }
    ///Bit 7 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch7(&mut self) -> AWD2CH7_W<AWD2CRrs> {
        AWD2CH7_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch8(&mut self) -> AWD2CH8_W<AWD2CRrs> {
        AWD2CH8_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch9(&mut self) -> AWD2CH9_W<AWD2CRrs> {
        AWD2CH9_W::new(self, 9)
    }
    ///Bit 10 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch10(&mut self) -> AWD2CH10_W<AWD2CRrs> {
        AWD2CH10_W::new(self, 10)
    }
    ///Bit 11 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch11(&mut self) -> AWD2CH11_W<AWD2CRrs> {
        AWD2CH11_W::new(self, 11)
    }
    ///Bit 12 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch12(&mut self) -> AWD2CH12_W<AWD2CRrs> {
        AWD2CH12_W::new(self, 12)
    }
    ///Bit 13 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch13(&mut self) -> AWD2CH13_W<AWD2CRrs> {
        AWD2CH13_W::new(self, 13)
    }
    ///Bit 14 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch14(&mut self) -> AWD2CH14_W<AWD2CRrs> {
        AWD2CH14_W::new(self, 14)
    }
    ///Bit 15 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch15(&mut self) -> AWD2CH15_W<AWD2CRrs> {
        AWD2CH15_W::new(self, 15)
    }
    ///Bit 16 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch16(&mut self) -> AWD2CH16_W<AWD2CRrs> {
        AWD2CH16_W::new(self, 16)
    }
    ///Bit 17 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch17(&mut self) -> AWD2CH17_W<AWD2CRrs> {
        AWD2CH17_W::new(self, 17)
    }
    ///Bit 18 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch18(&mut self) -> AWD2CH18_W<AWD2CRrs> {
        AWD2CH18_W::new(self, 18)
    }
}
/**Analog Watchdog 2 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#ADC3:AWD2CR)*/
pub struct AWD2CRrs;
impl crate::RegisterSpec for AWD2CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd2cr::R`](R) reader structure
impl crate::Readable for AWD2CRrs {}
///`write(|w| ..)` method takes [`awd2cr::W`](W) writer structure
impl crate::Writable for AWD2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AWD2CR to value 0
impl crate::Resettable for AWD2CRrs {
    const RESET_VALUE: u32 = 0;
}
