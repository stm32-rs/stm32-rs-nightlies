///Register `MODER` reader
pub type R = crate::R<MODERrs>;
///Register `MODER` writer
pub type W = crate::W<MODERrs>;
/**Port x configuration pin %s

Value on reset: 3*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Input mode (reset state)
    Input = 0,
    ///1: General purpose output mode
    Output = 1,
    ///2: Alternate function mode
    Alternate = 2,
    ///3: Analog mode
    Analog = 3,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODER(0-15)` reader - Port x configuration pin %s
pub type MODER_R = crate::FieldReader<MODE>;
impl MODER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::Input,
            1 => MODE::Output,
            2 => MODE::Alternate,
            3 => MODE::Analog,
            _ => unreachable!(),
        }
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE::Input
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE::Output
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODE::Alternate
    }
    ///Analog mode
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODE::Analog
    }
}
///Field `MODER(0-15)` writer - Port x configuration pin %s
pub type MODER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> MODER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Input)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Output)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Alternate)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Analog)
    }
}
impl R {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODER0` field.</div>
    #[inline(always)]
    pub fn moder(&self, n: u8) -> MODER_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MODER_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-15)
    #[inline(always)]
    pub fn moder_iter(&self) -> impl Iterator<Item = MODER_R> + '_ {
        (0..16).map(move |n| MODER_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn moder0(&self) -> MODER_R {
        MODER_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn moder1(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn moder2(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn moder3(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn moder4(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn moder5(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn moder6(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn moder7(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn moder8(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn moder9(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn moder10(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn moder11(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn moder12(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn moder13(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn moder14(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn moder15(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("moder0", &self.moder0())
            .field("moder1", &self.moder1())
            .field("moder2", &self.moder2())
            .field("moder3", &self.moder3())
            .field("moder4", &self.moder4())
            .field("moder5", &self.moder5())
            .field("moder6", &self.moder6())
            .field("moder7", &self.moder7())
            .field("moder8", &self.moder8())
            .field("moder9", &self.moder9())
            .field("moder10", &self.moder10())
            .field("moder11", &self.moder11())
            .field("moder12", &self.moder12())
            .field("moder13", &self.moder13())
            .field("moder14", &self.moder14())
            .field("moder15", &self.moder15())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODER0` field.</div>
    #[inline(always)]
    pub fn moder(&mut self, n: u8) -> MODER_W<'_, MODERrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MODER_W::new(self, n * 2)
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn moder0(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn moder1(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn moder2(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn moder3(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn moder4(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn moder5(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn moder6(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn moder7(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn moder8(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn moder9(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn moder10(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn moder11(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn moder12(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn moder13(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn moder14(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn moder15(&mut self) -> MODER_W<'_, MODERrs> {
        MODER_W::new(self, 30)
    }
}
/**GPIO port mode register

You can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#GPIOA:MODER)*/
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
///`read()` method returns [`moder::R`](R) reader structure
impl crate::Readable for MODERrs {}
///`write(|w| ..)` method takes [`moder::W`](W) writer structure
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MODER to value 0xabff_ffff
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0xabff_ffff;
}
