#[doc = "Register `D2CFGR` reader"]
pub type R = crate::R<D2CFGRrs>;
#[doc = "Register `D2CFGR` writer"]
pub type W = crate::W<D2CFGRrs>;
#[doc = "D2 domain APB1 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D2PPRE1 {
    #[doc = "0: rcc_hclk not divided"]
    Div1 = 0,
    #[doc = "4: rcc_hclk divided by 2"]
    Div2 = 4,
    #[doc = "5: rcc_hclk divided by 4"]
    Div4 = 5,
    #[doc = "6: rcc_hclk divided by 8"]
    Div8 = 6,
    #[doc = "7: rcc_hclk divided by 16"]
    Div16 = 7,
}
impl From<D2PPRE1> for u8 {
    #[inline(always)]
    fn from(variant: D2PPRE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for D2PPRE1 {
    type Ux = u8;
}
#[doc = "Field `D2PPRE1` reader - D2 domain APB1 prescaler"]
pub type D2PPRE1_R = crate::FieldReader<D2PPRE1>;
impl D2PPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<D2PPRE1> {
        match self.bits {
            0 => Some(D2PPRE1::Div1),
            4 => Some(D2PPRE1::Div2),
            5 => Some(D2PPRE1::Div4),
            6 => Some(D2PPRE1::Div8),
            7 => Some(D2PPRE1::Div16),
            _ => None,
        }
    }
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == D2PPRE1::Div1
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D2PPRE1::Div2
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D2PPRE1::Div4
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D2PPRE1::Div8
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D2PPRE1::Div16
    }
}
#[doc = "Field `D2PPRE1` writer - D2 domain APB1 prescaler"]
pub type D2PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, D2PPRE1>;
impl<'a, REG> D2PPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(D2PPRE1::Div16)
    }
}
#[doc = "Field `D2PPRE2` reader - D2 domain APB2 prescaler"]
pub use D2PPRE1_R as D2PPRE2_R;
#[doc = "Field `D2PPRE2` writer - D2 domain APB2 prescaler"]
pub use D2PPRE1_W as D2PPRE2_W;
impl R {
    #[doc = "Bits 4:6 - D2 domain APB1 prescaler"]
    #[inline(always)]
    pub fn d2ppre1(&self) -> D2PPRE1_R {
        D2PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - D2 domain APB2 prescaler"]
    #[inline(always)]
    pub fn d2ppre2(&self) -> D2PPRE2_R {
        D2PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - D2 domain APB1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d2ppre1(&mut self) -> D2PPRE1_W<D2CFGRrs> {
        D2PPRE1_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - D2 domain APB2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d2ppre2(&mut self) -> D2PPRE2_W<D2CFGRrs> {
        D2PPRE2_W::new(self, 8)
    }
}
#[doc = "RCC Domain 2 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2CFGRrs;
impl crate::RegisterSpec for D2CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2cfgr::R`](R) reader structure"]
impl crate::Readable for D2CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`d2cfgr::W`](W) writer structure"]
impl crate::Writable for D2CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D2CFGR to value 0"]
impl crate::Resettable for D2CFGRrs {
    const RESET_VALUE: u32 = 0;
}
