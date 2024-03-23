#[doc = "Register `GICD_SPENDSGIR3` reader"]
pub type R = crate::R<GICD_SPENDSGIR3rs>;
#[doc = "Register `GICD_SPENDSGIR3` writer"]
pub type W = crate::W<GICD_SPENDSGIR3rs>;
#[doc = "Field `SGI_SET_PENDING0` reader - SGI_SET_PENDING0"]
pub type SGI_SET_PENDING0_R = crate::FieldReader;
#[doc = "Field `SGI_SET_PENDING0` writer - SGI_SET_PENDING0"]
pub type SGI_SET_PENDING0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SGI_SET_PENDING1` reader - SGI_SET_PENDING1"]
pub type SGI_SET_PENDING1_R = crate::FieldReader;
#[doc = "Field `SGI_SET_PENDING1` writer - SGI_SET_PENDING1"]
pub type SGI_SET_PENDING1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SGI_SET_PENDING2` reader - SGI_SET_PENDING2"]
pub type SGI_SET_PENDING2_R = crate::FieldReader;
#[doc = "Field `SGI_SET_PENDING2` writer - SGI_SET_PENDING2"]
pub type SGI_SET_PENDING2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SGI_SET_PENDING3` reader - SGI_SET_PENDING3"]
pub type SGI_SET_PENDING3_R = crate::FieldReader;
#[doc = "Field `SGI_SET_PENDING3` writer - SGI_SET_PENDING3"]
pub type SGI_SET_PENDING3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - SGI_SET_PENDING0"]
    #[inline(always)]
    pub fn sgi_set_pending0(&self) -> SGI_SET_PENDING0_R {
        SGI_SET_PENDING0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - SGI_SET_PENDING1"]
    #[inline(always)]
    pub fn sgi_set_pending1(&self) -> SGI_SET_PENDING1_R {
        SGI_SET_PENDING1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SGI_SET_PENDING2"]
    #[inline(always)]
    pub fn sgi_set_pending2(&self) -> SGI_SET_PENDING2_R {
        SGI_SET_PENDING2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - SGI_SET_PENDING3"]
    #[inline(always)]
    pub fn sgi_set_pending3(&self) -> SGI_SET_PENDING3_R {
        SGI_SET_PENDING3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SGI_SET_PENDING0"]
    #[inline(always)]
    #[must_use]
    pub fn sgi_set_pending0(&mut self) -> SGI_SET_PENDING0_W<GICD_SPENDSGIR3rs> {
        SGI_SET_PENDING0_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - SGI_SET_PENDING1"]
    #[inline(always)]
    #[must_use]
    pub fn sgi_set_pending1(&mut self) -> SGI_SET_PENDING1_W<GICD_SPENDSGIR3rs> {
        SGI_SET_PENDING1_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - SGI_SET_PENDING2"]
    #[inline(always)]
    #[must_use]
    pub fn sgi_set_pending2(&mut self) -> SGI_SET_PENDING2_W<GICD_SPENDSGIR3rs> {
        SGI_SET_PENDING2_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - SGI_SET_PENDING3"]
    #[inline(always)]
    #[must_use]
    pub fn sgi_set_pending3(&mut self) -> SGI_SET_PENDING3_W<GICD_SPENDSGIR3rs> {
        SGI_SET_PENDING3_W::new(self, 24)
    }
}
#[doc = "For SGI x*4 to SGI x*4+3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spendsgir3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spendsgir3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SPENDSGIR3rs;
impl crate::RegisterSpec for GICD_SPENDSGIR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spendsgir3::R`](R) reader structure"]
impl crate::Readable for GICD_SPENDSGIR3rs {}
#[doc = "`write(|w| ..)` method takes [`gicd_spendsgir3::W`](W) writer structure"]
impl crate::Writable for GICD_SPENDSGIR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_SPENDSGIR3 to value 0"]
impl crate::Resettable for GICD_SPENDSGIR3rs {
    const RESET_VALUE: u32 = 0;
}
