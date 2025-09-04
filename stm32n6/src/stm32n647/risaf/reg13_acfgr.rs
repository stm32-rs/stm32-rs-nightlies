///Register `REG13_ACFGR` reader
pub type R = crate::R<REG13_ACFGRrs>;
///Register `REG13_ACFGR` writer
pub type W = crate::W<REG13_ACFGRrs>;
///Field `SREN` reader - subregion enable
pub type SREN_R = crate::BitReader;
///Field `SREN` writer - subregion enable
pub type SREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RLOCK` reader - resource lock
pub type RLOCK_R = crate::BitReader;
///Field `RLOCK` writer - resource lock
pub type RLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRCID` reader - subregion CID
pub type SRCID_R = crate::FieldReader;
///Field `SRCID` writer - subregion CID
pub type SRCID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SEC` reader - secure subregion
pub type SEC_R = crate::BitReader;
///Field `SEC` writer - secure subregion
pub type SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV` reader - privileged subregion
pub type PRIV_R = crate::BitReader;
///Field `PRIV` writer - privileged subregion
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDEN` reader - read enable
pub type RDEN_R = crate::BitReader;
///Field `RDEN` writer - read enable
pub type RDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WREN` reader - write enable
pub type WREN_R = crate::BitReader;
///Field `WREN` writer - write enable
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - subregion enable
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - resource lock
    #[inline(always)]
    pub fn rlock(&self) -> RLOCK_R {
        RLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - subregion CID
    #[inline(always)]
    pub fn srcid(&self) -> SRCID_R {
        SRCID_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - secure subregion
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - privileged subregion
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - read enable
    #[inline(always)]
    pub fn rden(&self) -> RDEN_R {
        RDEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - write enable
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG13_ACFGR")
            .field("sren", &self.sren())
            .field("rlock", &self.rlock())
            .field("srcid", &self.srcid())
            .field("sec", &self.sec())
            .field("priv_", &self.priv_())
            .field("rden", &self.rden())
            .field("wren", &self.wren())
            .finish()
    }
}
impl W {
    ///Bit 0 - subregion enable
    #[inline(always)]
    pub fn sren(&mut self) -> SREN_W<REG13_ACFGRrs> {
        SREN_W::new(self, 0)
    }
    ///Bit 1 - resource lock
    #[inline(always)]
    pub fn rlock(&mut self) -> RLOCK_W<REG13_ACFGRrs> {
        RLOCK_W::new(self, 1)
    }
    ///Bits 4:6 - subregion CID
    #[inline(always)]
    pub fn srcid(&mut self) -> SRCID_W<REG13_ACFGRrs> {
        SRCID_W::new(self, 4)
    }
    ///Bit 8 - secure subregion
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<REG13_ACFGRrs> {
        SEC_W::new(self, 8)
    }
    ///Bit 9 - privileged subregion
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<REG13_ACFGRrs> {
        PRIV_W::new(self, 9)
    }
    ///Bit 12 - read enable
    #[inline(always)]
    pub fn rden(&mut self) -> RDEN_W<REG13_ACFGRrs> {
        RDEN_W::new(self, 12)
    }
    ///Bit 13 - write enable
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<REG13_ACFGRrs> {
        WREN_W::new(self, 13)
    }
}
/**RISAF region 13 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg13_acfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_acfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_ACFGR)*/
pub struct REG13_ACFGRrs;
impl crate::RegisterSpec for REG13_ACFGRrs {
    type Ux = u32;
}
///`read()` method returns [`reg13_acfgr::R`](R) reader structure
impl crate::Readable for REG13_ACFGRrs {}
///`write(|w| ..)` method takes [`reg13_acfgr::W`](W) writer structure
impl crate::Writable for REG13_ACFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG13_ACFGR to value 0
impl crate::Resettable for REG13_ACFGRrs {}
