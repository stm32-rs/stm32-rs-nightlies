#[doc = "Register `CDCFGR1` reader"]
pub type R = crate::R<CDCFGR1rs>;
#[doc = "Register `CDCFGR1` writer"]
pub type W = crate::W<CDCFGR1rs>;
#[doc = "CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk.\n\nValue on reset: 0"]
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
#[doc = "Field `HPRE` reader - CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk."]
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
#[doc = "Field `HPRE` writer - CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk."]
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
#[doc = "CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDPPRE {
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
impl From<CDPPRE> for u8 {
    #[inline(always)]
    fn from(variant: CDPPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CDPPRE {
    type Ux = u8;
}
#[doc = "Field `CDPPRE` reader - CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)"]
pub type CDPPRE_R = crate::FieldReader<CDPPRE>;
impl CDPPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CDPPRE> {
        match self.bits {
            0 => Some(CDPPRE::Div1),
            4 => Some(CDPPRE::Div2),
            5 => Some(CDPPRE::Div4),
            6 => Some(CDPPRE::Div8),
            7 => Some(CDPPRE::Div16),
            _ => None,
        }
    }
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CDPPRE::Div1
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CDPPRE::Div2
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CDPPRE::Div4
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CDPPRE::Div8
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CDPPRE::Div16
    }
}
#[doc = "Field `CDPPRE` writer - CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)"]
pub type CDPPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CDPPRE>;
impl<'a, REG> CDPPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_hclk not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE::Div1)
    }
    #[doc = "rcc_hclk divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE::Div2)
    }
    #[doc = "rcc_hclk divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE::Div4)
    }
    #[doc = "rcc_hclk divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE::Div8)
    }
    #[doc = "rcc_hclk divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CDPPRE::Div16)
    }
}
#[doc = "Field `CDCPRE` reader - CPU domain core prescaler Set and reset by software to control the CPU domain CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)"]
pub use HPRE_R as CDCPRE_R;
#[doc = "Field `CDCPRE` writer - CPU domain core prescaler Set and reset by software to control the CPU domain CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)"]
pub use HPRE_W as CDCPRE_W;
impl R {
    #[doc = "Bits 0:3 - CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk."]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)"]
    #[inline(always)]
    pub fn cdppre(&self) -> CDPPRE_R {
        CDPPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - CPU domain core prescaler Set and reset by software to control the CPU domain CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)"]
    #[inline(always)]
    pub fn cdcpre(&self) -> CDCPRE_R {
        CDCPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CPU domain AHB prescaler Set and reset by software to control the division factor of rcc_hclk3 and rcc_aclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks. 0xxx: rcc_hclk3 = sys_cdcpre_ck (default after reset) Note: The clocks are divided by the new prescaler factor from1 to 16 periods of the slowest APB clock among rcc_pclk\\[4:1\\]
after HPRE update. Note: Note also that rcc_hclk3 = rcc_aclk."]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<CDCFGR1rs> {
        HPRE_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - CPU domain APB3 prescaler Set and reset by software to control the division factor of rcc_pclk3. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk3 after CDPPRE write. 0xx: rcc_pclk3 = rcc_hclk3 (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn cdppre(&mut self) -> CDPPRE_W<CDCFGR1rs> {
        CDPPRE_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - CPU domain core prescaler Set and reset by software to control the CPU domain CPU clock division factor. Changing this division ratio has an impact on the frequency of the CPU clock and all bus matrix clocks. After changing this prescaler value, it takes up to 16 periods of the slowest APB clock before the new division ratio is taken into account. The application can check if the new division factor is taken into account by reading back this register. 0xxx: sys_ck not divided (default after reset)"]
    #[inline(always)]
    #[must_use]
    pub fn cdcpre(&mut self) -> CDCPRE_W<CDCFGR1rs> {
        CDCPRE_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdcfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdcfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDCFGR1rs;
impl crate::RegisterSpec for CDCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdcfgr1::R`](R) reader structure"]
impl crate::Readable for CDCFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`cdcfgr1::W`](W) writer structure"]
impl crate::Writable for CDCFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDCFGR1 to value 0"]
impl crate::Resettable for CDCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
