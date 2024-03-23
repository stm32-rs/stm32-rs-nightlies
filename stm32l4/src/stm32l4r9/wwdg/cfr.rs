#[doc = "Register `CFR` reader"]
pub type R = crate::R<CFRrs>;
#[doc = "Register `CFR` writer"]
pub type W = crate::W<CFRrs>;
#[doc = "Field `W` reader - 7-bit window value"]
pub type W_R = crate::FieldReader;
#[doc = "Field `W` writer - 7-bit window value"]
pub type W_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Timer base\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDGTB {
    #[doc = "0: Counter clock (PCLK1 div 4096) div 1"]
    Div1 = 0,
    #[doc = "1: Counter clock (PCLK1 div 4096) div 2"]
    Div2 = 1,
    #[doc = "2: Counter clock (PCLK1 div 4096) div 4"]
    Div4 = 2,
    #[doc = "3: Counter clock (PCLK1 div 4096) div 8"]
    Div8 = 3,
}
impl From<WDGTB> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDGTB {
    type Ux = u8;
}
#[doc = "Field `WDGTB` reader - Timer base"]
pub type WDGTB_R = crate::FieldReader<WDGTB>;
impl WDGTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDGTB {
        match self.bits {
            0 => WDGTB::Div1,
            1 => WDGTB::Div2,
            2 => WDGTB::Div4,
            3 => WDGTB::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == WDGTB::Div1
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WDGTB::Div2
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WDGTB::Div4
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WDGTB::Div8
    }
}
#[doc = "Field `WDGTB` writer - Timer base"]
pub type WDGTB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WDGTB>;
impl<'a, REG> WDGTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div1)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div2)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div4)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div8)
    }
}
#[doc = "Early wakeup interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIW {
    #[doc = "1: interrupt occurs whenever the counter reaches the value 0x40"]
    Enable = 1,
}
impl From<EWIW> for bool {
    #[inline(always)]
    fn from(variant: EWIW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWI` reader - Early wakeup interrupt"]
pub type EWI_R = crate::BitReader<EWIW>;
impl EWI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EWIW> {
        match self.bits {
            true => Some(EWIW::Enable),
            _ => None,
        }
    }
    #[doc = "interrupt occurs whenever the counter reaches the value 0x40"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWIW::Enable
    }
}
#[doc = "Field `EWI` writer - Early wakeup interrupt"]
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG, EWIW>;
impl<'a, REG> EWI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt occurs whenever the counter reaches the value 0x40"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EWIW::Enable)
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Timer base"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> W_W<CFRrs> {
        W_W::new(self, 0)
    }
    #[doc = "Bits 7:8 - Timer base"]
    #[inline(always)]
    #[must_use]
    pub fn wdgtb(&mut self) -> WDGTB_W<CFRrs> {
        WDGTB_W::new(self, 7)
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EWI_W<CFRrs> {
        EWI_W::new(self, 9)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfr::R`](R) reader structure"]
impl crate::Readable for CFRrs {}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFR to value 0x7f"]
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u32 = 0x7f;
}
