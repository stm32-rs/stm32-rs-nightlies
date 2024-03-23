#[doc = "Register `DBPCR` reader"]
pub type R = crate::R<DBPCRrs>;
#[doc = "Register `DBPCR` writer"]
pub type W = crate::W<DBPCRrs>;
#[doc = "Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    #[doc = "0: Write access to backup domain disabled"]
    Disabled = 0,
    #[doc = "1: Write access to backup domain enabled"]
    Enabled = 1,
}
impl From<DBP> for bool {
    #[inline(always)]
    fn from(variant: DBP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBP` reader - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_R = crate::BitReader<DBP>;
impl DBP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBP {
        match self.bits {
            false => DBP::Disabled,
            true => DBP::Enabled,
        }
    }
    #[doc = "Write access to backup domain disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP::Disabled
    }
    #[doc = "Write access to backup domain enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP::Enabled
    }
}
#[doc = "Field `DBP` writer - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write access to backup domain disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Disabled)
    }
    #[doc = "Write access to backup domain enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<DBPCRrs> {
        DBP_W::new(self, 0)
    }
}
#[doc = "PWR Backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBPCRrs;
impl crate::RegisterSpec for DBPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbpcr::R`](R) reader structure"]
impl crate::Readable for DBPCRrs {}
#[doc = "`write(|w| ..)` method takes [`dbpcr::W`](W) writer structure"]
impl crate::Writable for DBPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBPCR to value 0"]
impl crate::Resettable for DBPCRrs {
    const RESET_VALUE: u32 = 0;
}
