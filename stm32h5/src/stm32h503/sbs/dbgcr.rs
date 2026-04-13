///Register `DBGCR` reader
pub type R = crate::R<DBGCRrs>;
///Register `DBGCR` writer
pub type W = crate::W<DBGCRrs>;
/**access port unlock Write 0xB4 to this bitfield to open the device access port.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AP_UNLOCK {
    ///180: Device access port unlocked
    Unlocked = 180,
}
impl From<AP_UNLOCK> for u8 {
    #[inline(always)]
    fn from(variant: AP_UNLOCK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AP_UNLOCK {
    type Ux = u8;
}
impl crate::IsEnum for AP_UNLOCK {}
///Field `AP_UNLOCK` reader - access port unlock Write 0xB4 to this bitfield to open the device access port.
pub type AP_UNLOCK_R = crate::FieldReader<AP_UNLOCK>;
impl AP_UNLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<AP_UNLOCK> {
        match self.bits {
            180 => Some(AP_UNLOCK::Unlocked),
            _ => None,
        }
    }
    ///Device access port unlocked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == AP_UNLOCK::Unlocked
    }
}
///Field `AP_UNLOCK` writer - access port unlock Write 0xB4 to this bitfield to open the device access port.
pub type AP_UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8, AP_UNLOCK>;
impl<'a, REG> AP_UNLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Device access port unlocked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(AP_UNLOCK::Unlocked)
    }
}
/**debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBG_UNLOCK {
    ///180: Debug unlocked when HDPLSR:HDPL is equal to DBG_AUTH_HDPL
    Unlocked = 180,
}
impl From<DBG_UNLOCK> for u8 {
    #[inline(always)]
    fn from(variant: DBG_UNLOCK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBG_UNLOCK {
    type Ux = u8;
}
impl crate::IsEnum for DBG_UNLOCK {}
///Field `DBG_UNLOCK` reader - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
pub type DBG_UNLOCK_R = crate::FieldReader<DBG_UNLOCK>;
impl DBG_UNLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBG_UNLOCK> {
        match self.bits {
            180 => Some(DBG_UNLOCK::Unlocked),
            _ => None,
        }
    }
    ///Debug unlocked when HDPLSR:HDPL is equal to DBG_AUTH_HDPL
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DBG_UNLOCK::Unlocked
    }
}
///Field `DBG_UNLOCK` writer - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
pub type DBG_UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DBG_UNLOCK>;
impl<'a, REG> DBG_UNLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Debug unlocked when HDPLSR:HDPL is equal to DBG_AUTH_HDPL
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_UNLOCK::Unlocked)
    }
}
/**authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBG_AUTH_HDPL {
    ///81: Protection level to be used to execute and protect immutable Root of Trust (IROT) stage
    Hdpl1 = 81,
    ///111: Protection level to be used to execute the application
    Hdpl3 = 111,
    ///138: Protection level to be used to execute and protect an updatable Root of Trust (UROT) stage
    Hdpl2 = 138,
}
impl From<DBG_AUTH_HDPL> for u8 {
    #[inline(always)]
    fn from(variant: DBG_AUTH_HDPL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBG_AUTH_HDPL {
    type Ux = u8;
}
impl crate::IsEnum for DBG_AUTH_HDPL {}
///Field `DBG_AUTH_HDPL` reader - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
pub type DBG_AUTH_HDPL_R = crate::FieldReader<DBG_AUTH_HDPL>;
impl DBG_AUTH_HDPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBG_AUTH_HDPL> {
        match self.bits {
            81 => Some(DBG_AUTH_HDPL::Hdpl1),
            111 => Some(DBG_AUTH_HDPL::Hdpl3),
            138 => Some(DBG_AUTH_HDPL::Hdpl2),
            _ => None,
        }
    }
    ///Protection level to be used to execute and protect immutable Root of Trust (IROT) stage
    #[inline(always)]
    pub fn is_hdpl1(&self) -> bool {
        *self == DBG_AUTH_HDPL::Hdpl1
    }
    ///Protection level to be used to execute the application
    #[inline(always)]
    pub fn is_hdpl3(&self) -> bool {
        *self == DBG_AUTH_HDPL::Hdpl3
    }
    ///Protection level to be used to execute and protect an updatable Root of Trust (UROT) stage
    #[inline(always)]
    pub fn is_hdpl2(&self) -> bool {
        *self == DBG_AUTH_HDPL::Hdpl2
    }
}
///Field `DBG_AUTH_HDPL` writer - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
pub type DBG_AUTH_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DBG_AUTH_HDPL>;
impl<'a, REG> DBG_AUTH_HDPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Protection level to be used to execute and protect immutable Root of Trust (IROT) stage
    #[inline(always)]
    pub fn hdpl1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_AUTH_HDPL::Hdpl1)
    }
    ///Protection level to be used to execute the application
    #[inline(always)]
    pub fn hdpl3(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_AUTH_HDPL::Hdpl3)
    }
    ///Protection level to be used to execute and protect an updatable Root of Trust (UROT) stage
    #[inline(always)]
    pub fn hdpl2(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_AUTH_HDPL::Hdpl2)
    }
}
impl R {
    ///Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port.
    #[inline(always)]
    pub fn ap_unlock(&self) -> AP_UNLOCK_R {
        AP_UNLOCK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
    #[inline(always)]
    pub fn dbg_unlock(&self) -> DBG_UNLOCK_R {
        DBG_UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
    #[inline(always)]
    pub fn dbg_auth_hdpl(&self) -> DBG_AUTH_HDPL_R {
        DBG_AUTH_HDPL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGCR")
            .field("ap_unlock", &self.ap_unlock())
            .field("dbg_unlock", &self.dbg_unlock())
            .field("dbg_auth_hdpl", &self.dbg_auth_hdpl())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port.
    #[inline(always)]
    pub fn ap_unlock(&mut self) -> AP_UNLOCK_W<'_, DBGCRrs> {
        AP_UNLOCK_W::new(self, 0)
    }
    ///Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
    #[inline(always)]
    pub fn dbg_unlock(&mut self) -> DBG_UNLOCK_W<'_, DBGCRrs> {
        DBG_UNLOCK_W::new(self, 8)
    }
    ///Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
    #[inline(always)]
    pub fn dbg_auth_hdpl(&mut self) -> DBG_AUTH_HDPL_W<'_, DBGCRrs> {
        DBG_AUTH_HDPL_W::new(self, 16)
    }
}
/**SBS debug control register

You can [`read`](crate::Reg::read) this register and get [`dbgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:DBGCR)*/
pub struct DBGCRrs;
impl crate::RegisterSpec for DBGCRrs {
    type Ux = u32;
}
///`read()` method returns [`dbgcr::R`](R) reader structure
impl crate::Readable for DBGCRrs {}
///`write(|w| ..)` method takes [`dbgcr::W`](W) writer structure
impl crate::Writable for DBGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGCR to value 0
impl crate::Resettable for DBGCRrs {}
