#[doc = "Register `MODER` reader"]
pub type R = crate::R<MODERrs>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<MODERrs>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODER3 {
    #[doc = "0: Input mode (reset state)"]
    Input = 0,
    #[doc = "1: General purpose output mode"]
    Output = 1,
    #[doc = "2: Alternate function mode"]
    Alternate = 2,
    #[doc = "3: Analog mode"]
    Analog = 3,
}
impl From<MODER3> for u8 {
    #[inline(always)]
    fn from(variant: MODER3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODER3 {
    type Ux = u8;
}
#[doc = "Field `MODER3` reader - Port x configuration bits (y = 0..15)"]
pub type MODER3_R = crate::FieldReader<MODER3>;
impl MODER3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODER3 {
        match self.bits {
            0 => MODER3::Input,
            1 => MODER3::Output,
            2 => MODER3::Alternate,
            3 => MODER3::Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODER3::Input
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODER3::Output
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == MODER3::Alternate
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODER3::Analog
    }
}
#[doc = "Field `MODER3` writer - Port x configuration bits (y = 0..15)"]
pub type MODER3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODER3>;
impl<'a, REG> MODER3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode (reset state)"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODER3::Input)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODER3::Output)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(MODER3::Alternate)
    }
    #[doc = "Analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(MODER3::Analog)
    }
}
impl R {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER3_W<MODERrs> {
        MODER3_W::new(self, 6)
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
#[doc = "`reset()` method sets MODER to value 0xc0"]
impl crate::Resettable for MODERrs {
    const RESET_VALUE: u32 = 0xc0;
}
