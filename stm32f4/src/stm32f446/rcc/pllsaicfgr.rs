#[doc = "Register `PLLSAICFGR` reader"]
pub type R = crate::R<PLLSAICFGRrs>;
#[doc = "Register `PLLSAICFGR` writer"]
pub type W = crate::W<PLLSAICFGRrs>;
#[doc = "Field `PLLSAIM` reader - Division factor for audio PLLSAI input clock"]
pub type PLLSAIM_R = crate::FieldReader;
#[doc = "Field `PLLSAIM` writer - Division factor for audio PLLSAI input clock"]
pub type PLLSAIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLLSAIN` reader - PLLSAI division factor for VCO"]
pub type PLLSAIN_R = crate::FieldReader<u16>;
#[doc = "Field `PLLSAIN` writer - PLLSAI division factor for VCO"]
pub type PLLSAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "PLLSAI division factor for 48 MHz clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSAIP {
    #[doc = "0: PLL*P=2"]
    Div2 = 0,
    #[doc = "1: PLL*P=4"]
    Div4 = 1,
    #[doc = "2: PLL*P=6"]
    Div6 = 2,
    #[doc = "3: PLL*P=8"]
    Div8 = 3,
}
impl From<PLLSAIP> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSAIP {
    type Ux = u8;
}
#[doc = "Field `PLLSAIP` reader - PLLSAI division factor for 48 MHz clock"]
pub type PLLSAIP_R = crate::FieldReader<PLLSAIP>;
impl PLLSAIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSAIP {
        match self.bits {
            0 => PLLSAIP::Div2,
            1 => PLLSAIP::Div4,
            2 => PLLSAIP::Div6,
            3 => PLLSAIP::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "PLL*P=2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIP::Div2
    }
    #[doc = "PLL*P=4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIP::Div4
    }
    #[doc = "PLL*P=6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAIP::Div6
    }
    #[doc = "PLL*P=8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIP::Div8
    }
}
#[doc = "Field `PLLSAIP` writer - PLLSAI division factor for 48 MHz clock"]
pub type PLLSAIP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLLSAIP>;
impl<'a, REG> PLLSAIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL*P=2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIP::Div2)
    }
    #[doc = "PLL*P=4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIP::Div4)
    }
    #[doc = "PLL*P=6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIP::Div6)
    }
    #[doc = "PLL*P=8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSAIP::Div8)
    }
}
#[doc = "Field `PLLSAIQ` reader - PLLSAI division factor for SAIs clock"]
pub type PLLSAIQ_R = crate::FieldReader;
#[doc = "Field `PLLSAIQ` writer - PLLSAI division factor for SAIs clock"]
pub type PLLSAIQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Division factor for audio PLLSAI input clock"]
    #[inline(always)]
    pub fn pllsaim(&self) -> PLLSAIM_R {
        PLLSAIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    pub fn pllsain(&self) -> PLLSAIN_R {
        PLLSAIN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - PLLSAI division factor for 48 MHz clock"]
    #[inline(always)]
    pub fn pllsaip(&self) -> PLLSAIP_R {
        PLLSAIP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAIs clock"]
    #[inline(always)]
    pub fn pllsaiq(&self) -> PLLSAIQ_R {
        PLLSAIQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Division factor for audio PLLSAI input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaim(&mut self) -> PLLSAIM_W<PLLSAICFGRrs> {
        PLLSAIM_W::new(self, 0)
    }
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn pllsain(&mut self) -> PLLSAIN_W<PLLSAICFGRrs> {
        PLLSAIN_W::new(self, 6)
    }
    #[doc = "Bits 16:17 - PLLSAI division factor for 48 MHz clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaip(&mut self) -> PLLSAIP_W<PLLSAICFGRrs> {
        PLLSAIP_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAIs clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllsaiq(&mut self) -> PLLSAIQ_W<PLLSAICFGRrs> {
        PLLSAIQ_W::new(self, 24)
    }
}
#[doc = "PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsaicfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsaicfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLSAICFGRrs;
impl crate::RegisterSpec for PLLSAICFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsaicfgr::R`](R) reader structure"]
impl crate::Readable for PLLSAICFGRrs {}
#[doc = "`write(|w| ..)` method takes [`pllsaicfgr::W`](W) writer structure"]
impl crate::Writable for PLLSAICFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLSAICFGR to value 0x2400_3000"]
impl crate::Resettable for PLLSAICFGRrs {
    const RESET_VALUE: u32 = 0x2400_3000;
}
