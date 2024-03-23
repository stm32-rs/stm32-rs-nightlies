#[doc = "Register `L1WVPCR` reader"]
pub type R = crate::R<L1WVPCRrs>;
#[doc = "Register `L1WVPCR` writer"]
pub type W = crate::W<L1WVPCRrs>;
#[doc = "Field `WVSTPOS` reader - Window Vertical Start Position"]
pub type WVSTPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WVSTPOS` writer - Window Vertical Start Position"]
pub type WVSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `WVSPPOS` reader - Window Vertical Stop Position"]
pub type WVSPPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WVSPPOS` writer - Window Vertical Stop Position"]
pub type WVSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Window Vertical Start Position"]
    #[inline(always)]
    #[must_use]
    pub fn wvstpos(&mut self) -> WVSTPOS_W<L1WVPCRrs> {
        WVSTPOS_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Window Vertical Stop Position"]
    #[inline(always)]
    #[must_use]
    pub fn wvsppos(&mut self) -> WVSPPOS_W<L1WVPCRrs> {
        WVSPPOS_W::new(self, 16)
    }
}
#[doc = "LTDC Layer Window Vertical Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1wvpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1wvpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1WVPCRrs;
impl crate::RegisterSpec for L1WVPCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1wvpcr::R`](R) reader structure"]
impl crate::Readable for L1WVPCRrs {}
#[doc = "`write(|w| ..)` method takes [`l1wvpcr::W`](W) writer structure"]
impl crate::Writable for L1WVPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1WVPCR to value 0"]
impl crate::Resettable for L1WVPCRrs {
    const RESET_VALUE: u32 = 0;
}
