#[doc = "Register `BKP_DR%s` reader"]
pub type R = crate::R<BKP_DRrs>;
#[doc = "Register `BKP_DR%s` writer"]
pub type W = crate::W<BKP_DRrs>;
#[doc = "Field `D` reader - Backup data"]
pub type D_R = crate::FieldReader<u16>;
#[doc = "Field `D` writer - Backup data"]
pub type D_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d(&mut self) -> D_W<BKP_DRrs> {
        D_W::new(self, 0)
    }
}
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkp_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkp_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKP_DRrs;
impl crate::RegisterSpec for BKP_DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp_dr::R`](R) reader structure"]
impl crate::Readable for BKP_DRrs {}
#[doc = "`write(|w| ..)` method takes [`bkp_dr::W`](W) writer structure"]
impl crate::Writable for BKP_DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BKP_DR%s to value 0"]
impl crate::Resettable for BKP_DRrs {
    const RESET_VALUE: u32 = 0;
}
