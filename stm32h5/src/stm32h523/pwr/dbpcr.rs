///Register `DBPCR` reader
pub type R = crate::R<DBPCRrs>;
///Register `DBPCR` writer
pub type W = crate::W<DBPCRrs>;
/**Disable Backup domain write protection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP {
    ///0: Write access to backup domain disabled
    Disabled = 0,
    ///1: Write access to backup domain enabled
    Enabled = 1,
}
impl From<DBP> for bool {
    #[inline(always)]
    fn from(variant: DBP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBP` reader - Disable Backup domain write protection
pub type DBP_R = crate::BitReader<DBP>;
impl DBP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBP {
        match self.bits {
            false => DBP::Disabled,
            true => DBP::Enabled,
        }
    }
    ///Write access to backup domain disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP::Disabled
    }
    ///Write access to backup domain enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP::Enabled
    }
}
///Field `DBP` writer - Disable Backup domain write protection
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write access to backup domain disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Disabled)
    }
    ///Write access to backup domain enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBP::Enabled)
    }
}
impl R {
    ///Bit 0 - Disable Backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBPCR").field("dbp", &self.dbp()).finish()
    }
}
impl W {
    ///Bit 0 - Disable Backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<'_, DBPCRrs> {
        DBP_W::new(self, 0)
    }
}
/**PWR Backup domain control register

You can [`read`](crate::Reg::read) this register and get [`dbpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#PWR:DBPCR)*/
pub struct DBPCRrs;
impl crate::RegisterSpec for DBPCRrs {
    type Ux = u32;
}
///`read()` method returns [`dbpcr::R`](R) reader structure
impl crate::Readable for DBPCRrs {}
///`write(|w| ..)` method takes [`dbpcr::W`](W) writer structure
impl crate::Writable for DBPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBPCR to value 0
impl crate::Resettable for DBPCRrs {}
