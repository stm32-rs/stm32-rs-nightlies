#[doc = "Register `PLLI2SCFGR` reader"]
pub type R = crate::R<PLLI2SCFGRrs>;
#[doc = "Register `PLLI2SCFGR` writer"]
pub type W = crate::W<PLLI2SCFGRrs>;
#[doc = "Field `PLLI2SN` reader - PLLI2S multiplication factor for VCO"]
pub type PLLI2SN_R = crate::FieldReader<u16>;
#[doc = "Field `PLLI2SN` writer - PLLI2S multiplication factor for VCO"]
pub type PLLI2SN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "PLLI2S division factor for SPDIFRX clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLI2SP {
    #[doc = "0: PLL*P=2"]
    Div2 = 0,
    #[doc = "1: PLL*P=4"]
    Div4 = 1,
    #[doc = "2: PLL*P=6"]
    Div6 = 2,
    #[doc = "3: PLL*P=8"]
    Div8 = 3,
}
impl From<PLLI2SP> for u8 {
    #[inline(always)]
    fn from(variant: PLLI2SP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLI2SP {
    type Ux = u8;
}
#[doc = "Field `PLLI2SP` reader - PLLI2S division factor for SPDIFRX clock"]
pub type PLLI2SP_R = crate::FieldReader<PLLI2SP>;
impl PLLI2SP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLI2SP {
        match self.bits {
            0 => PLLI2SP::Div2,
            1 => PLLI2SP::Div4,
            2 => PLLI2SP::Div6,
            3 => PLLI2SP::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "PLL*P=2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLI2SP::Div2
    }
    #[doc = "PLL*P=4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLI2SP::Div4
    }
    #[doc = "PLL*P=6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLI2SP::Div6
    }
    #[doc = "PLL*P=8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLI2SP::Div8
    }
}
#[doc = "Field `PLLI2SP` writer - PLLI2S division factor for SPDIFRX clock"]
pub type PLLI2SP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLLI2SP>;
impl<'a, REG> PLLI2SP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL*P=2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SP::Div2)
    }
    #[doc = "PLL*P=4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SP::Div4)
    }
    #[doc = "PLL*P=6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SP::Div6)
    }
    #[doc = "PLL*P=8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLI2SP::Div8)
    }
}
#[doc = "Field `PLLI2SQ` reader - PLLI2S division factor for SAI1 clock"]
pub type PLLI2SQ_R = crate::FieldReader;
#[doc = "Field `PLLI2SQ` writer - PLLI2S division factor for SAI1 clock"]
pub type PLLI2SQ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLI2SR` reader - PLLI2S division factor for I2S clocks"]
pub type PLLI2SR_R = crate::FieldReader;
#[doc = "Field `PLLI2SR` writer - PLLI2S division factor for I2S clocks"]
pub type PLLI2SR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2sn(&self) -> PLLI2SN_R {
        PLLI2SN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - PLLI2S division factor for SPDIFRX clock"]
    #[inline(always)]
    pub fn plli2sp(&self) -> PLLI2SP_R {
        PLLI2SP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sq(&self) -> PLLI2SQ_R {
        PLLI2SQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2sr(&self) -> PLLI2SR_R {
        PLLI2SR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sn(&mut self) -> PLLI2SN_W<PLLI2SCFGRrs> {
        PLLI2SN_W::new(self, 6)
    }
    #[doc = "Bits 16:17 - PLLI2S division factor for SPDIFRX clock"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sp(&mut self) -> PLLI2SP_W<PLLI2SCFGRrs> {
        PLLI2SP_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sq(&mut self) -> PLLI2SQ_W<PLLI2SCFGRrs> {
        PLLI2SQ_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    #[must_use]
    pub fn plli2sr(&mut self) -> PLLI2SR_W<PLLI2SCFGRrs> {
        PLLI2SR_W::new(self, 28)
    }
}
#[doc = "PLLI2S configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plli2scfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plli2scfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLI2SCFGRrs;
impl crate::RegisterSpec for PLLI2SCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plli2scfgr::R`](R) reader structure"]
impl crate::Readable for PLLI2SCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`plli2scfgr::W`](W) writer structure"]
impl crate::Writable for PLLI2SCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLI2SCFGR to value 0x2000_3000"]
impl crate::Resettable for PLLI2SCFGRrs {
    const RESET_VALUE: u32 = 0x2000_3000;
}
