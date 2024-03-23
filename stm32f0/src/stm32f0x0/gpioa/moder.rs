#[doc = "Register `MODER` reader"]
pub type R = crate::R<MODERrs>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<MODERrs>;
#[doc = "Port x configuration pin %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODER0 {
    #[doc = "0: Input mode (reset state)"]
    Input = 0,
    #[doc = "1: General purpose output mode"]
    Output = 1,
    #[doc = "2: Alternate function mode"]
    Alternate = 2,
    #[doc = "3: Analog mode"]
    Analog = 3,
}
impl From<MODER0> for u8 {
    #[inline(always)]
    fn from(variant: MODER0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODER0 {
    type Ux = u8;
}
#[doc = "Field `MODER(0-15)` reader - Port x configuration pin %s"]
pub type MODER_R = crate::FieldReader<MODER0>;
impl MODER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODER0 {
        match self.bits {
            0 => MODER0::Input,
            1 => MODER0::Output,
            2 => MODER0::Alternate,
            3 => MODER0::Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODER0::Input
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODER0::Output
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODER0::Alternate
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODER0::Analog
    }
}
#[doc = "Field `MODER(0-15)` writer - Port x configuration pin %s"]
pub type MODER_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODER0>;
impl<'a, REG> MODER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0::Input)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0::Output)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0::Alternate)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0::Analog)
    }
}
impl R {
    #[doc = "Port x configuration pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `MODER0` field"]
    #[inline(always)]
    pub fn moder(&self, n: u8) -> MODER_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MODER_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port x configuration pin (0-15)"]
    #[inline(always)]
    pub fn moder_iter(&self) -> impl Iterator<Item = MODER_R> + '_ {
        (0..16).map(move |n| MODER_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Port x configuration pin 0"]
    #[inline(always)]
    pub fn moder0(&self) -> MODER_R {
        MODER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration pin 1"]
    #[inline(always)]
    pub fn moder1(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration pin 2"]
    #[inline(always)]
    pub fn moder2(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration pin 3"]
    #[inline(always)]
    pub fn moder3(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration pin 4"]
    #[inline(always)]
    pub fn moder4(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration pin 5"]
    #[inline(always)]
    pub fn moder5(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration pin 6"]
    #[inline(always)]
    pub fn moder6(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration pin 7"]
    #[inline(always)]
    pub fn moder7(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration pin 8"]
    #[inline(always)]
    pub fn moder8(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration pin 9"]
    #[inline(always)]
    pub fn moder9(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration pin 10"]
    #[inline(always)]
    pub fn moder10(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration pin 11"]
    #[inline(always)]
    pub fn moder11(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration pin 12"]
    #[inline(always)]
    pub fn moder12(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration pin 13"]
    #[inline(always)]
    pub fn moder13(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration pin 14"]
    #[inline(always)]
    pub fn moder14(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration pin 15"]
    #[inline(always)]
    pub fn moder15(&self) -> MODER_R {
        MODER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Port x configuration pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `MODER0` field"]
    #[inline(always)]
    #[must_use]
    pub fn moder(&mut self, n: u8) -> MODER_W<MODERrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MODER_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - Port x configuration pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn moder0(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn moder1(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn moder2(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn moder4(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn moder5(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn moder6(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn moder7(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn moder8(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn moder9(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn moder10(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn moder11(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn moder12(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn moder13(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn moder14(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn moder15(&mut self) -> MODER_W<MODERrs> {
        MODER_W::new(self, 30)
    }
}
#[doc = "GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODERrs;
impl crate::RegisterSpec for MODERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for MODERrs {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for MODERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODER to value 0x2800_0000"]
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0x2800_0000;
}
