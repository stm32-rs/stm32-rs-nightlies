#[doc = "Register `EXTCFGR` reader"]
pub type R = crate::R<EXTCFGRrs>;
#[doc = "Register `EXTCFGR` writer"]
pub type W = crate::W<EXTCFGRrs>;
#[doc = "HCLK3 shared prescaler (AHB3, Flash, and SRAM2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHDHPRE {
    #[doc = "0: SYSCLK not divided"]
    Div1 = 0,
    #[doc = "1: SYSCLK divided by 3"]
    Div3 = 1,
    #[doc = "2: SYSCLK divided by 5"]
    Div5 = 2,
    #[doc = "5: SYSCLK divided by 6"]
    Div6 = 5,
    #[doc = "6: SYSCLK divided by 10"]
    Div10 = 6,
    #[doc = "7: SYSCLK divided by 32"]
    Div32 = 7,
    #[doc = "8: SYSCLK divided by 2"]
    Div2 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    Div4 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    Div8 = 10,
    #[doc = "11: SYSCLK divided by 16"]
    Div16 = 11,
    #[doc = "12: SYSCLK divided by 64"]
    Div64 = 12,
    #[doc = "13: SYSCLK divided by 128"]
    Div128 = 13,
    #[doc = "14: SYSCLK divided by 128"]
    Div256 = 14,
    #[doc = "15: SYSCLK divided by 512"]
    Div512 = 15,
}
impl From<SHDHPRE> for u8 {
    #[inline(always)]
    fn from(variant: SHDHPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SHDHPRE {
    type Ux = u8;
}
#[doc = "Field `SHDHPRE` reader - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)"]
pub type SHDHPRE_R = crate::FieldReader<SHDHPRE>;
impl SHDHPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SHDHPRE> {
        match self.bits {
            0 => Some(SHDHPRE::Div1),
            1 => Some(SHDHPRE::Div3),
            2 => Some(SHDHPRE::Div5),
            5 => Some(SHDHPRE::Div6),
            6 => Some(SHDHPRE::Div10),
            7 => Some(SHDHPRE::Div32),
            8 => Some(SHDHPRE::Div2),
            9 => Some(SHDHPRE::Div4),
            10 => Some(SHDHPRE::Div8),
            11 => Some(SHDHPRE::Div16),
            12 => Some(SHDHPRE::Div64),
            13 => Some(SHDHPRE::Div128),
            14 => Some(SHDHPRE::Div256),
            15 => Some(SHDHPRE::Div512),
            _ => None,
        }
    }
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SHDHPRE::Div1
    }
    #[doc = "SYSCLK divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == SHDHPRE::Div3
    }
    #[doc = "SYSCLK divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == SHDHPRE::Div5
    }
    #[doc = "SYSCLK divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == SHDHPRE::Div6
    }
    #[doc = "SYSCLK divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == SHDHPRE::Div10
    }
    #[doc = "SYSCLK divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == SHDHPRE::Div32
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SHDHPRE::Div2
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SHDHPRE::Div4
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SHDHPRE::Div8
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == SHDHPRE::Div16
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == SHDHPRE::Div64
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == SHDHPRE::Div128
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == SHDHPRE::Div256
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == SHDHPRE::Div512
    }
}
#[doc = "Field `SHDHPRE` writer - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)"]
pub type SHDHPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SHDHPRE>;
impl<'a, REG> SHDHPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div1)
    }
    #[doc = "SYSCLK divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div3)
    }
    #[doc = "SYSCLK divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div5)
    }
    #[doc = "SYSCLK divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div6)
    }
    #[doc = "SYSCLK divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div10)
    }
    #[doc = "SYSCLK divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div32)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div128)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div512)
    }
}
#[doc = "HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHDHPREF {
    #[doc = "0: HCLK3 prescaler value not yet applied"]
    NotApplied = 0,
    #[doc = "1: HCLK3 prescaler value applied"]
    Applied = 1,
}
impl From<SHDHPREF> for bool {
    #[inline(always)]
    fn from(variant: SHDHPREF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHDHPREF` reader - HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)"]
pub type SHDHPREF_R = crate::BitReader<SHDHPREF>;
impl SHDHPREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SHDHPREF {
        match self.bits {
            false => SHDHPREF::NotApplied,
            true => SHDHPREF::Applied,
        }
    }
    #[doc = "HCLK3 prescaler value not yet applied"]
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == SHDHPREF::NotApplied
    }
    #[doc = "HCLK3 prescaler value applied"]
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == SHDHPREF::Applied
    }
}
impl R {
    #[doc = "Bits 0:3 - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)"]
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)"]
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)"]
    #[inline(always)]
    #[must_use]
    pub fn shdhpre(&mut self) -> SHDHPRE_W<EXTCFGRrs> {
        SHDHPRE_W::new(self, 0)
    }
}
#[doc = "Extended clock recovery register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTCFGRrs;
impl crate::RegisterSpec for EXTCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extcfgr::R`](R) reader structure"]
impl crate::Readable for EXTCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`extcfgr::W`](W) writer structure"]
impl crate::Writable for EXTCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTCFGR to value 0x0003_0000"]
impl crate::Resettable for EXTCFGRrs {
    const RESET_VALUE: u32 = 0x0003_0000;
}
