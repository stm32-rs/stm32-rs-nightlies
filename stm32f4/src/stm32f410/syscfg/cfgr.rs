#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `FMPI2C1_SCL` reader - FMPI2C1_SCL"]
pub type FMPI2C1_SCL_R = crate::BitReader;
#[doc = "Field `FMPI2C1_SCL` writer - FMPI2C1_SCL"]
pub type FMPI2C1_SCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMPI2C1_SDA` reader - FMPI2C1_SDA"]
pub type FMPI2C1_SDA_R = crate::BitReader;
#[doc = "Field `FMPI2C1_SDA` writer - FMPI2C1_SDA"]
pub type FMPI2C1_SDA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FMPI2C1_SCL"]
    #[inline(always)]
    pub fn fmpi2c1_scl(&self) -> FMPI2C1_SCL_R {
        FMPI2C1_SCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FMPI2C1_SDA"]
    #[inline(always)]
    pub fn fmpi2c1_sda(&self) -> FMPI2C1_SDA_R {
        FMPI2C1_SDA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMPI2C1_SCL"]
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1_scl(&mut self) -> FMPI2C1_SCL_W<CFGRrs> {
        FMPI2C1_SCL_W::new(self, 0)
    }
    #[doc = "Bit 1 - FMPI2C1_SDA"]
    #[inline(always)]
    #[must_use]
    pub fn fmpi2c1_sda(&mut self) -> FMPI2C1_SDA_W<CFGRrs> {
        FMPI2C1_SDA_W::new(self, 1)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
