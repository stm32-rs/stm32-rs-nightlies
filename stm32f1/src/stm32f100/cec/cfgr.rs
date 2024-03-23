#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `PE` reader - Peripheral enable"]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - Peripheral enable"]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Interrupt enable"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt enable"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTEM` reader - Bit timing error mode"]
pub type BTEM_R = crate::BitReader;
#[doc = "Field `BTEM` writer - Bit timing error mode"]
pub type BTEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPEM` reader - Bit period error mode"]
pub type BPEM_R = crate::BitReader;
#[doc = "Field `BPEM` writer - Bit period error mode"]
pub type BPEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit timing error mode"]
    #[inline(always)]
    pub fn btem(&self) -> BTEM_R {
        BTEM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit period error mode"]
    #[inline(always)]
    pub fn bpem(&self) -> BPEM_R {
        BPEM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<CFGRrs> {
        PE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CFGRrs> {
        IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit timing error mode"]
    #[inline(always)]
    #[must_use]
    pub fn btem(&mut self) -> BTEM_W<CFGRrs> {
        BTEM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit period error mode"]
    #[inline(always)]
    #[must_use]
    pub fn bpem(&mut self) -> BPEM_W<CFGRrs> {
        BPEM_W::new(self, 3)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
