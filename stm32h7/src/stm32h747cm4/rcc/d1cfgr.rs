#[doc = "Register `D1CFGR` reader"]
pub type R = crate::R<D1CFGRrs>;
#[doc = "Register `D1CFGR` writer"]
pub type W = crate::W<D1CFGRrs>;
#[doc = "D1 domain AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    #[doc = "0: sys_ck not divided"]
    Div1 = 0,
    #[doc = "8: sys_ck divided by 2"]
    Div2 = 8,
    #[doc = "9: sys_ck divided by 4"]
    Div4 = 9,
    #[doc = "10: sys_ck divided by 8"]
    Div8 = 10,
    #[doc = "11: sys_ck divided by 16"]
    Div16 = 11,
    #[doc = "12: sys_ck divided by 64"]
    Div64 = 12,
    #[doc = "13: sys_ck divided by 128"]
    Div128 = 13,
    #[doc = "14: sys_ck divided by 256"]
    Div256 = 14,
    #[doc = "15: sys_ck divided by 512"]
    Div512 = 15,
}
impl From<HPRE> for u8 {
    #[inline(always)]
    fn from(variant: HPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE {
    type Ux = u8;
}
#[doc = "Field `HPRE` reader - D1 domain AHB prescaler"]
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HPRE> {
        match self.bits {
            0 => Some(HPRE::Div1),
            8 => Some(HPRE::Div2),
            9 => Some(HPRE::Div4),
            10 => Some(HPRE::Div8),
            11 => Some(HPRE::Div16),
            12 => Some(HPRE::Div64),
            13 => Some(HPRE::Div128),
            14 => Some(HPRE::Div256),
            15 => Some(HPRE::Div512),
            _ => None,
        }
    }
    #[doc = "sys_ck not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE::Div1
    }
    #[doc = "sys_ck divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE::Div2
    }
    #[doc = "sys_ck divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE::Div4
    }
    #[doc = "sys_ck divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE::Div8
    }
    #[doc = "sys_ck divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE::Div16
    }
    #[doc = "sys_ck divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE::Div64
    }
    #[doc = "sys_ck divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE::Div128
    }
    #[doc = "sys_ck divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE::Div256
    }
    #[doc = "sys_ck divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE::Div512
    }
}
#[doc = "Field `HPRE` writer - D1 domain AHB prescaler"]
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "sys_ck not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div1)
    }
    #[doc = "sys_ck divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div2)
    }
    #[doc = "sys_ck divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div4)
    }
    #[doc = "sys_ck divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div8)
    }
    #[doc = "sys_ck divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div16)
    }
    #[doc = "sys_ck divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div64)
    }
    #[doc = "sys_ck divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div128)
    }
    #[doc = "sys_ck divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div256)
    }
    #[doc = "sys_ck divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::Div512)
    }
}
#[doc = "D1 domain APB3 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D1PPRE {
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
impl From<D1PPRE> for u8 {
    #[inline(always)]
    fn from(variant: D1PPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for D1PPRE {
    type Ux = u8;
}
#[doc = "Field `D1PPRE` reader - D1 domain APB3 prescaler"]
pub type D1PPRE_R = crate::FieldReader<D1PPRE>;
impl D1PPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<D1PPRE> {
        match self.bits {
            0 => Some(D1PPRE::Div1),
            4 => Some(D1PPRE::Div2),
            5 => Some(D1PPRE::Div4),
            6 => Some(D1PPRE::Div8),
            7 => Some(D1PPRE::Div16),
            _ => None,
        }
    }
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == D1PPRE::Div1
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == D1PPRE::Div2
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == D1PPRE::Div4
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == D1PPRE::Div8
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == D1PPRE::Div16
    }
}
#[doc = "Field `D1PPRE` writer - D1 domain APB3 prescaler"]
pub type D1PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, D1PPRE>;
impl<'a, REG> D1PPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(D1PPRE::Div16)
    }
}
#[doc = "Field `D1CPRE` reader - D1 domain Core prescaler"]
pub use HPRE_R as D1CPRE_R;
#[doc = "Field `D1CPRE` writer - D1 domain Core prescaler"]
pub use HPRE_W as D1CPRE_W;
impl R {
    #[doc = "Bits 0:3 - D1 domain AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - D1 domain APB3 prescaler"]
    #[inline(always)]
    pub fn d1ppre(&self) -> D1PPRE_R {
        D1PPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - D1 domain Core prescaler"]
    #[inline(always)]
    pub fn d1cpre(&self) -> D1CPRE_R {
        D1CPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - D1 domain AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<D1CFGRrs> {
        HPRE_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - D1 domain APB3 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d1ppre(&mut self) -> D1PPRE_W<D1CFGRrs> {
        D1PPRE_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - D1 domain Core prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d1cpre(&mut self) -> D1CPRE_W<D1CFGRrs> {
        D1CPRE_W::new(self, 8)
    }
}
#[doc = "RCC Domain 1 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1CFGRrs;
impl crate::RegisterSpec for D1CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1cfgr::R`](R) reader structure"]
impl crate::Readable for D1CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`d1cfgr::W`](W) writer structure"]
impl crate::Writable for D1CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D1CFGR to value 0"]
impl crate::Resettable for D1CFGRrs {
    const RESET_VALUE: u32 = 0;
}
