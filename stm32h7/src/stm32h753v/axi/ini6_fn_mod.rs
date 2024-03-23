#[doc = "Register `INI6_FN_MOD` reader"]
pub type R = crate::R<INI6_FN_MODrs>;
#[doc = "Register `INI6_FN_MOD` writer"]
pub type W = crate::W<INI6_FN_MODrs>;
#[doc = "Override ASIB read issuing capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READ_ISS_OVERRIDE {
    #[doc = "0: Normal ASIB read issuing capability"]
    Normal = 0,
    #[doc = "1: Force ASIB read issuing capability to 1"]
    Force1 = 1,
}
impl From<READ_ISS_OVERRIDE> for bool {
    #[inline(always)]
    fn from(variant: READ_ISS_OVERRIDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_ISS_OVERRIDE` reader - Override ASIB read issuing capability"]
pub type READ_ISS_OVERRIDE_R = crate::BitReader<READ_ISS_OVERRIDE>;
impl READ_ISS_OVERRIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> READ_ISS_OVERRIDE {
        match self.bits {
            false => READ_ISS_OVERRIDE::Normal,
            true => READ_ISS_OVERRIDE::Force1,
        }
    }
    #[doc = "Normal ASIB read issuing capability"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == READ_ISS_OVERRIDE::Normal
    }
    #[doc = "Force ASIB read issuing capability to 1"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == READ_ISS_OVERRIDE::Force1
    }
}
#[doc = "Field `READ_ISS_OVERRIDE` writer - Override ASIB read issuing capability"]
pub type READ_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG, READ_ISS_OVERRIDE>;
impl<'a, REG> READ_ISS_OVERRIDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal ASIB read issuing capability"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(READ_ISS_OVERRIDE::Normal)
    }
    #[doc = "Force ASIB read issuing capability to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut crate::W<REG> {
        self.variant(READ_ISS_OVERRIDE::Force1)
    }
}
#[doc = "Override ASIB write issuing capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRITE_ISS_OVERRIDE {
    #[doc = "0: Normal ASIB write issuing capability"]
    Normal = 0,
    #[doc = "1: Force ASIB write issuing capability to 1"]
    Force1 = 1,
}
impl From<WRITE_ISS_OVERRIDE> for bool {
    #[inline(always)]
    fn from(variant: WRITE_ISS_OVERRIDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Override ASIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader<WRITE_ISS_OVERRIDE>;
impl WRITE_ISS_OVERRIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRITE_ISS_OVERRIDE {
        match self.bits {
            false => WRITE_ISS_OVERRIDE::Normal,
            true => WRITE_ISS_OVERRIDE::Force1,
        }
    }
    #[doc = "Normal ASIB write issuing capability"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WRITE_ISS_OVERRIDE::Normal
    }
    #[doc = "Force ASIB write issuing capability to 1"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == WRITE_ISS_OVERRIDE::Force1
    }
}
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Override ASIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG, WRITE_ISS_OVERRIDE>;
impl<'a, REG> WRITE_ISS_OVERRIDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal ASIB write issuing capability"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(WRITE_ISS_OVERRIDE::Normal)
    }
    #[doc = "Force ASIB write issuing capability to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut crate::W<REG> {
        self.variant(WRITE_ISS_OVERRIDE::Force1)
    }
}
impl R {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<INI6_FN_MODrs> {
        READ_ISS_OVERRIDE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<INI6_FN_MODrs> {
        WRITE_ISS_OVERRIDE_W::new(self, 1)
    }
}
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini6_fn_mod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini6_fn_mod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INI6_FN_MODrs;
impl crate::RegisterSpec for INI6_FN_MODrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ini6_fn_mod::R`](R) reader structure"]
impl crate::Readable for INI6_FN_MODrs {}
#[doc = "`write(|w| ..)` method takes [`ini6_fn_mod::W`](W) writer structure"]
impl crate::Writable for INI6_FN_MODrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INI6_FN_MOD to value 0x04"]
impl crate::Resettable for INI6_FN_MODrs {
    const RESET_VALUE: u32 = 0x04;
}
