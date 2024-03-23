#[doc = "Register `D3CFGR` reader"]
pub type R = crate::R<D3CFGRrs>;
#[doc = "Register `D3CFGR` writer"]
pub type W = crate::W<D3CFGRrs>;
#[doc = "D3 domain APB4 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D3PPRE {
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
impl From<D3PPRE> for u8 {
    #[inline(always)]
    fn from(variant: D3PPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for D3PPRE {
    type Ux = u8;
}
#[doc = "Field `D3PPRE` reader - D3 domain APB4 prescaler"]
pub type D3PPRE_R = crate::FieldReader<D3PPRE>;
impl D3PPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<D3PPRE> {
        match self.bits {
            0 => Some(D3PPRE::Div1),
            4 => Some(D3PPRE::Div2),
            5 => Some(D3PPRE::Div4),
            6 => Some(D3PPRE::Div8),
            7 => Some(D3PPRE::Div16),
            _ => None,
        }
    }
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == D3PPRE::Div1
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D3PPRE::Div2
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D3PPRE::Div4
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D3PPRE::Div8
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D3PPRE::Div16
    }
}
#[doc = "Field `D3PPRE` writer - D3 domain APB4 prescaler"]
pub type D3PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, D3PPRE>;
impl<'a, REG> D3PPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(D3PPRE::Div16)
    }
}
impl R {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    pub fn d3ppre(&self) -> D3PPRE_R {
        D3PPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d3ppre(&mut self) -> D3PPRE_W<D3CFGRrs> {
        D3PPRE_W::new(self, 4)
    }
}
#[doc = "RCC Domain 3 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3CFGRrs;
impl crate::RegisterSpec for D3CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3cfgr::R`](R) reader structure"]
impl crate::Readable for D3CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`d3cfgr::W`](W) writer structure"]
impl crate::Writable for D3CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D3CFGR to value 0"]
impl crate::Resettable for D3CFGRrs {
    const RESET_VALUE: u32 = 0;
}
