#[doc = "Register `CDCFGR2` reader"]
pub type R = crate::R<CDCFGR2rs>;
#[doc = "Register `CDCFGR2` writer"]
pub type W = crate::W<CDCFGR2rs>;
#[doc = "CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDPPRE1 {
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
impl From<CDPPRE1> for u8 {
    #[inline(always)]
    fn from(variant: CDPPRE1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDPPRE1 {
    type Ux = u8;
}
#[doc = "Field `CDPPRE1` reader - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
pub type CDPPRE1_R = crate::FieldReader<CDPPRE1>;
impl CDPPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CDPPRE1> {
        match self.bits {
            0 => Some(CDPPRE1::Div1),
            4 => Some(CDPPRE1::Div2),
            5 => Some(CDPPRE1::Div4),
            6 => Some(CDPPRE1::Div8),
            7 => Some(CDPPRE1::Div16),
            _ => None,
        }
    }
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CDPPRE1::Div1
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CDPPRE1::Div2
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CDPPRE1::Div4
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CDPPRE1::Div8
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CDPPRE1::Div16
    }
}
#[doc = "Field `CDPPRE1` writer - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
pub type CDPPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CDPPRE1>;
impl<'a, REG> CDPPRE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE1::Div16)
    }
}
#[doc = "Field `CDPPRE2` reader - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)"]
pub use CDPPRE1_R as CDPPRE2_R;
#[doc = "Field `CDPPRE2` writer - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)"]
pub use CDPPRE1_W as CDPPRE2_W;
impl R {
    #[doc = "Bits 4:6 - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    pub fn cdppre1(&self) -> CDPPRE1_R {
        CDPPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    pub fn cdppre2(&self) -> CDPPRE2_R {
        CDPPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - CPU domain APB1 prescaler Set and reset by software to control the CPU domain APB1 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE1 write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn cdppre1(&mut self) -> CDPPRE1_W<CDCFGR2rs> {
        CDPPRE1_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - CPU domain APB2 prescaler Set and reset by software to control the CPU domain APB2 clock division factor. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk1 after CDPPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1 (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn cdppre2(&mut self) -> CDPPRE2_W<CDCFGR2rs> {
        CDPPRE2_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDCFGR2rs;
impl crate::RegisterSpec for CDCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdcfgr2::R`](R) reader structure"]
impl crate::Readable for CDCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cdcfgr2::W`](W) writer structure"]
impl crate::Writable for CDCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDCFGR2 to value 0"]
impl crate::Resettable for CDCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
