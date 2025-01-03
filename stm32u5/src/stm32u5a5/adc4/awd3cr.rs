///Register `AWD3CR` reader
pub type R = crate::R<AWD3CRrs>;
///Register `AWD3CR` writer
pub type W = crate::W<AWD3CRrs>;
/**AWD3CH0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH0 {
    ///0: ADC analog input channel x is not monitored by AWDy
    Disabled = 0,
    ///1: ADC analog input channel x is monitored by AWDy
    Enabled = 1,
}
impl From<AWD3CH0> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH0) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD3CH0` reader - AWD3CH0
pub type AWD3CH0_R = crate::BitReader<AWD3CH0>;
impl AWD3CH0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH0 {
        match self.bits {
            false => AWD3CH0::Disabled,
            true => AWD3CH0::Enabled,
        }
    }
    ///ADC analog input channel x is not monitored by AWDy
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD3CH0::Disabled
    }
    ///ADC analog input channel x is monitored by AWDy
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD3CH0::Enabled
    }
}
///Field `AWD3CH0` writer - AWD3CH0
pub type AWD3CH0_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH0>;
impl<'a, REG> AWD3CH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog input channel x is not monitored by AWDy
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::Disabled)
    }
    ///ADC analog input channel x is monitored by AWDy
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::Enabled)
    }
}
///Field `AWD3CH1` reader - AWD3CH1
pub use AWD3CH0_R as AWD3CH1_R;
///Field `AWD3CH2` reader - AWD3CH2
pub use AWD3CH0_R as AWD3CH2_R;
///Field `AWD3CH3` reader - AWD3CH3
pub use AWD3CH0_R as AWD3CH3_R;
///Field `AWD3CH4` reader - AWD3CH4
pub use AWD3CH0_R as AWD3CH4_R;
///Field `AWD3CH5` reader - AWD3CH5
pub use AWD3CH0_R as AWD3CH5_R;
///Field `AWD3CH6` reader - AWD3CH6
pub use AWD3CH0_R as AWD3CH6_R;
///Field `AWD3CH7` reader - AWD3CH7
pub use AWD3CH0_R as AWD3CH7_R;
///Field `AWD3CH8` reader - AWD3CH8
pub use AWD3CH0_R as AWD3CH8_R;
///Field `AWD3CH9` reader - AWD3CH9
pub use AWD3CH0_R as AWD3CH9_R;
///Field `AWD3CH10` reader - AWD3CH10
pub use AWD3CH0_R as AWD3CH10_R;
///Field `AWD3CH11` reader - AWD3CH11
pub use AWD3CH0_R as AWD3CH11_R;
///Field `AWD3CH12` reader - AWD3CH12
pub use AWD3CH0_R as AWD3CH12_R;
///Field `AWD3CH13` reader - AWD3CH13
pub use AWD3CH0_R as AWD3CH13_R;
///Field `AWD3CH14` reader - AWD3CH14
pub use AWD3CH0_R as AWD3CH14_R;
///Field `AWD3CH15` reader - AWD3CH15
pub use AWD3CH0_R as AWD3CH15_R;
///Field `AWD3CH16` reader - AWD3CH16
pub use AWD3CH0_R as AWD3CH16_R;
///Field `AWD3CH17` reader - AWD3CH17
pub use AWD3CH0_R as AWD3CH17_R;
///Field `AWD3CH18` reader - AWD3CH18
pub use AWD3CH0_R as AWD3CH18_R;
///Field `AWD3CH19` reader - AWD3CH19
pub use AWD3CH0_R as AWD3CH19_R;
///Field `AWD3CH20` reader - AWD3CH20
pub use AWD3CH0_R as AWD3CH20_R;
///Field `AWD3CH21` reader - AWD3CH21
pub use AWD3CH0_R as AWD3CH21_R;
///Field `AWD3CH22` reader - AWD3CH22
pub use AWD3CH0_R as AWD3CH22_R;
///Field `AWD3CH23` reader - AWD3CH23
pub use AWD3CH0_R as AWD3CH23_R;
///Field `AWD3CH1` writer - AWD3CH1
pub use AWD3CH0_W as AWD3CH1_W;
///Field `AWD3CH2` writer - AWD3CH2
pub use AWD3CH0_W as AWD3CH2_W;
///Field `AWD3CH3` writer - AWD3CH3
pub use AWD3CH0_W as AWD3CH3_W;
///Field `AWD3CH4` writer - AWD3CH4
pub use AWD3CH0_W as AWD3CH4_W;
///Field `AWD3CH5` writer - AWD3CH5
pub use AWD3CH0_W as AWD3CH5_W;
///Field `AWD3CH6` writer - AWD3CH6
pub use AWD3CH0_W as AWD3CH6_W;
///Field `AWD3CH7` writer - AWD3CH7
pub use AWD3CH0_W as AWD3CH7_W;
///Field `AWD3CH8` writer - AWD3CH8
pub use AWD3CH0_W as AWD3CH8_W;
///Field `AWD3CH9` writer - AWD3CH9
pub use AWD3CH0_W as AWD3CH9_W;
///Field `AWD3CH10` writer - AWD3CH10
pub use AWD3CH0_W as AWD3CH10_W;
///Field `AWD3CH11` writer - AWD3CH11
pub use AWD3CH0_W as AWD3CH11_W;
///Field `AWD3CH12` writer - AWD3CH12
pub use AWD3CH0_W as AWD3CH12_W;
///Field `AWD3CH13` writer - AWD3CH13
pub use AWD3CH0_W as AWD3CH13_W;
///Field `AWD3CH14` writer - AWD3CH14
pub use AWD3CH0_W as AWD3CH14_W;
///Field `AWD3CH15` writer - AWD3CH15
pub use AWD3CH0_W as AWD3CH15_W;
///Field `AWD3CH16` writer - AWD3CH16
pub use AWD3CH0_W as AWD3CH16_W;
///Field `AWD3CH17` writer - AWD3CH17
pub use AWD3CH0_W as AWD3CH17_W;
///Field `AWD3CH18` writer - AWD3CH18
pub use AWD3CH0_W as AWD3CH18_W;
///Field `AWD3CH19` writer - AWD3CH19
pub use AWD3CH0_W as AWD3CH19_W;
///Field `AWD3CH20` writer - AWD3CH20
pub use AWD3CH0_W as AWD3CH20_W;
///Field `AWD3CH21` writer - AWD3CH21
pub use AWD3CH0_W as AWD3CH21_W;
///Field `AWD3CH22` writer - AWD3CH22
pub use AWD3CH0_W as AWD3CH22_W;
///Field `AWD3CH23` writer - AWD3CH23
pub use AWD3CH0_W as AWD3CH23_W;
impl R {
    ///Bit 0 - AWD3CH0
    #[inline(always)]
    pub fn awd3ch0(&self) -> AWD3CH0_R {
        AWD3CH0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AWD3CH1
    #[inline(always)]
    pub fn awd3ch1(&self) -> AWD3CH1_R {
        AWD3CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AWD3CH2
    #[inline(always)]
    pub fn awd3ch2(&self) -> AWD3CH2_R {
        AWD3CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AWD3CH3
    #[inline(always)]
    pub fn awd3ch3(&self) -> AWD3CH3_R {
        AWD3CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AWD3CH4
    #[inline(always)]
    pub fn awd3ch4(&self) -> AWD3CH4_R {
        AWD3CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AWD3CH5
    #[inline(always)]
    pub fn awd3ch5(&self) -> AWD3CH5_R {
        AWD3CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AWD3CH6
    #[inline(always)]
    pub fn awd3ch6(&self) -> AWD3CH6_R {
        AWD3CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AWD3CH7
    #[inline(always)]
    pub fn awd3ch7(&self) -> AWD3CH7_R {
        AWD3CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AWD3CH8
    #[inline(always)]
    pub fn awd3ch8(&self) -> AWD3CH8_R {
        AWD3CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AWD3CH9
    #[inline(always)]
    pub fn awd3ch9(&self) -> AWD3CH9_R {
        AWD3CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AWD3CH10
    #[inline(always)]
    pub fn awd3ch10(&self) -> AWD3CH10_R {
        AWD3CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AWD3CH11
    #[inline(always)]
    pub fn awd3ch11(&self) -> AWD3CH11_R {
        AWD3CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AWD3CH12
    #[inline(always)]
    pub fn awd3ch12(&self) -> AWD3CH12_R {
        AWD3CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AWD3CH13
    #[inline(always)]
    pub fn awd3ch13(&self) -> AWD3CH13_R {
        AWD3CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AWD3CH14
    #[inline(always)]
    pub fn awd3ch14(&self) -> AWD3CH14_R {
        AWD3CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AWD3CH15
    #[inline(always)]
    pub fn awd3ch15(&self) -> AWD3CH15_R {
        AWD3CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AWD3CH16
    #[inline(always)]
    pub fn awd3ch16(&self) -> AWD3CH16_R {
        AWD3CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AWD3CH17
    #[inline(always)]
    pub fn awd3ch17(&self) -> AWD3CH17_R {
        AWD3CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - AWD3CH18
    #[inline(always)]
    pub fn awd3ch18(&self) -> AWD3CH18_R {
        AWD3CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - AWD3CH19
    #[inline(always)]
    pub fn awd3ch19(&self) -> AWD3CH19_R {
        AWD3CH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - AWD3CH20
    #[inline(always)]
    pub fn awd3ch20(&self) -> AWD3CH20_R {
        AWD3CH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - AWD3CH21
    #[inline(always)]
    pub fn awd3ch21(&self) -> AWD3CH21_R {
        AWD3CH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - AWD3CH22
    #[inline(always)]
    pub fn awd3ch22(&self) -> AWD3CH22_R {
        AWD3CH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - AWD3CH23
    #[inline(always)]
    pub fn awd3ch23(&self) -> AWD3CH23_R {
        AWD3CH23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD3CR")
            .field("awd3ch0", &self.awd3ch0())
            .field("awd3ch23", &self.awd3ch23())
            .field("awd3ch22", &self.awd3ch22())
            .field("awd3ch21", &self.awd3ch21())
            .field("awd3ch20", &self.awd3ch20())
            .field("awd3ch19", &self.awd3ch19())
            .field("awd3ch18", &self.awd3ch18())
            .field("awd3ch17", &self.awd3ch17())
            .field("awd3ch16", &self.awd3ch16())
            .field("awd3ch15", &self.awd3ch15())
            .field("awd3ch14", &self.awd3ch14())
            .field("awd3ch13", &self.awd3ch13())
            .field("awd3ch12", &self.awd3ch12())
            .field("awd3ch11", &self.awd3ch11())
            .field("awd3ch10", &self.awd3ch10())
            .field("awd3ch9", &self.awd3ch9())
            .field("awd3ch8", &self.awd3ch8())
            .field("awd3ch7", &self.awd3ch7())
            .field("awd3ch6", &self.awd3ch6())
            .field("awd3ch5", &self.awd3ch5())
            .field("awd3ch4", &self.awd3ch4())
            .field("awd3ch3", &self.awd3ch3())
            .field("awd3ch2", &self.awd3ch2())
            .field("awd3ch1", &self.awd3ch1())
            .finish()
    }
}
impl W {
    ///Bit 0 - AWD3CH0
    #[inline(always)]
    pub fn awd3ch0(&mut self) -> AWD3CH0_W<AWD3CRrs> {
        AWD3CH0_W::new(self, 0)
    }
    ///Bit 1 - AWD3CH1
    #[inline(always)]
    pub fn awd3ch1(&mut self) -> AWD3CH1_W<AWD3CRrs> {
        AWD3CH1_W::new(self, 1)
    }
    ///Bit 2 - AWD3CH2
    #[inline(always)]
    pub fn awd3ch2(&mut self) -> AWD3CH2_W<AWD3CRrs> {
        AWD3CH2_W::new(self, 2)
    }
    ///Bit 3 - AWD3CH3
    #[inline(always)]
    pub fn awd3ch3(&mut self) -> AWD3CH3_W<AWD3CRrs> {
        AWD3CH3_W::new(self, 3)
    }
    ///Bit 4 - AWD3CH4
    #[inline(always)]
    pub fn awd3ch4(&mut self) -> AWD3CH4_W<AWD3CRrs> {
        AWD3CH4_W::new(self, 4)
    }
    ///Bit 5 - AWD3CH5
    #[inline(always)]
    pub fn awd3ch5(&mut self) -> AWD3CH5_W<AWD3CRrs> {
        AWD3CH5_W::new(self, 5)
    }
    ///Bit 6 - AWD3CH6
    #[inline(always)]
    pub fn awd3ch6(&mut self) -> AWD3CH6_W<AWD3CRrs> {
        AWD3CH6_W::new(self, 6)
    }
    ///Bit 7 - AWD3CH7
    #[inline(always)]
    pub fn awd3ch7(&mut self) -> AWD3CH7_W<AWD3CRrs> {
        AWD3CH7_W::new(self, 7)
    }
    ///Bit 8 - AWD3CH8
    #[inline(always)]
    pub fn awd3ch8(&mut self) -> AWD3CH8_W<AWD3CRrs> {
        AWD3CH8_W::new(self, 8)
    }
    ///Bit 9 - AWD3CH9
    #[inline(always)]
    pub fn awd3ch9(&mut self) -> AWD3CH9_W<AWD3CRrs> {
        AWD3CH9_W::new(self, 9)
    }
    ///Bit 10 - AWD3CH10
    #[inline(always)]
    pub fn awd3ch10(&mut self) -> AWD3CH10_W<AWD3CRrs> {
        AWD3CH10_W::new(self, 10)
    }
    ///Bit 11 - AWD3CH11
    #[inline(always)]
    pub fn awd3ch11(&mut self) -> AWD3CH11_W<AWD3CRrs> {
        AWD3CH11_W::new(self, 11)
    }
    ///Bit 12 - AWD3CH12
    #[inline(always)]
    pub fn awd3ch12(&mut self) -> AWD3CH12_W<AWD3CRrs> {
        AWD3CH12_W::new(self, 12)
    }
    ///Bit 13 - AWD3CH13
    #[inline(always)]
    pub fn awd3ch13(&mut self) -> AWD3CH13_W<AWD3CRrs> {
        AWD3CH13_W::new(self, 13)
    }
    ///Bit 14 - AWD3CH14
    #[inline(always)]
    pub fn awd3ch14(&mut self) -> AWD3CH14_W<AWD3CRrs> {
        AWD3CH14_W::new(self, 14)
    }
    ///Bit 15 - AWD3CH15
    #[inline(always)]
    pub fn awd3ch15(&mut self) -> AWD3CH15_W<AWD3CRrs> {
        AWD3CH15_W::new(self, 15)
    }
    ///Bit 16 - AWD3CH16
    #[inline(always)]
    pub fn awd3ch16(&mut self) -> AWD3CH16_W<AWD3CRrs> {
        AWD3CH16_W::new(self, 16)
    }
    ///Bit 17 - AWD3CH17
    #[inline(always)]
    pub fn awd3ch17(&mut self) -> AWD3CH17_W<AWD3CRrs> {
        AWD3CH17_W::new(self, 17)
    }
    ///Bit 18 - AWD3CH18
    #[inline(always)]
    pub fn awd3ch18(&mut self) -> AWD3CH18_W<AWD3CRrs> {
        AWD3CH18_W::new(self, 18)
    }
    ///Bit 19 - AWD3CH19
    #[inline(always)]
    pub fn awd3ch19(&mut self) -> AWD3CH19_W<AWD3CRrs> {
        AWD3CH19_W::new(self, 19)
    }
    ///Bit 20 - AWD3CH20
    #[inline(always)]
    pub fn awd3ch20(&mut self) -> AWD3CH20_W<AWD3CRrs> {
        AWD3CH20_W::new(self, 20)
    }
    ///Bit 21 - AWD3CH21
    #[inline(always)]
    pub fn awd3ch21(&mut self) -> AWD3CH21_W<AWD3CRrs> {
        AWD3CH21_W::new(self, 21)
    }
    ///Bit 22 - AWD3CH22
    #[inline(always)]
    pub fn awd3ch22(&mut self) -> AWD3CH22_W<AWD3CRrs> {
        AWD3CH22_W::new(self, 22)
    }
    ///Bit 23 - AWD3CH23
    #[inline(always)]
    pub fn awd3ch23(&mut self) -> AWD3CH23_W<AWD3CRrs> {
        AWD3CH23_W::new(self, 23)
    }
}
/**ADC Analog Watchdog 3 Configuration register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#ADC4:AWD3CR)*/
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
