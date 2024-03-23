#[doc = "Register `CR5` reader"]
pub type R = crate::R<CR5rs>;
#[doc = "Register `CR5` writer"]
pub type W = crate::W<CR5rs>;
#[doc = "Field `R1MODE` reader - Main regular range 1 mode"]
pub type R1MODE_R = crate::BitReader;
#[doc = "Field `R1MODE` writer - Main regular range 1 mode"]
pub type R1MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main regular range 1 mode"]
    #[inline(always)]
    pub fn r1mode(&self) -> R1MODE_R {
        R1MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main regular range 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn r1mode(&mut self) -> R1MODE_W<CR5rs> {
        R1MODE_W::new(self, 0)
    }
}
#[doc = "Power control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr5::R`](R) reader structure"]
impl crate::Readable for CR5rs {}
#[doc = "`write(|w| ..)` method takes [`cr5::W`](W) writer structure"]
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR5 to value 0x0100"]
impl crate::Resettable for CR5rs {
    const RESET_VALUE: u32 = 0x0100;
}
