///Register `CPENDSGIR1` reader
pub type R = crate::R<CPENDSGIR1rs>;
///Register `CPENDSGIR1` writer
pub type W = crate::W<CPENDSGIR1rs>;
///Field `SGI_CLEAR_PENDING0` reader - SGI_CLEAR_PENDING0
pub type SGI_CLEAR_PENDING0_R = crate::FieldReader;
///Field `SGI_CLEAR_PENDING0` writer - SGI_CLEAR_PENDING0
pub type SGI_CLEAR_PENDING0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SGI_CLEAR_PENDING1` reader - SGI_CLEAR_PENDING1
pub type SGI_CLEAR_PENDING1_R = crate::FieldReader;
///Field `SGI_CLEAR_PENDING1` writer - SGI_CLEAR_PENDING1
pub type SGI_CLEAR_PENDING1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SGI_CLEAR_PENDING2` reader - SGI_CLEAR_PENDING2
pub type SGI_CLEAR_PENDING2_R = crate::FieldReader;
///Field `SGI_CLEAR_PENDING2` writer - SGI_CLEAR_PENDING2
pub type SGI_CLEAR_PENDING2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SGI_CLEAR_PENDING3` reader - SGI_CLEAR_PENDING3
pub type SGI_CLEAR_PENDING3_R = crate::FieldReader;
///Field `SGI_CLEAR_PENDING3` writer - SGI_CLEAR_PENDING3
pub type SGI_CLEAR_PENDING3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - SGI_CLEAR_PENDING0
    #[inline(always)]
    pub fn sgi_clear_pending0(&self) -> SGI_CLEAR_PENDING0_R {
        SGI_CLEAR_PENDING0_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - SGI_CLEAR_PENDING1
    #[inline(always)]
    pub fn sgi_clear_pending1(&self) -> SGI_CLEAR_PENDING1_R {
        SGI_CLEAR_PENDING1_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - SGI_CLEAR_PENDING2
    #[inline(always)]
    pub fn sgi_clear_pending2(&self) -> SGI_CLEAR_PENDING2_R {
        SGI_CLEAR_PENDING2_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:25 - SGI_CLEAR_PENDING3
    #[inline(always)]
    pub fn sgi_clear_pending3(&self) -> SGI_CLEAR_PENDING3_R {
        SGI_CLEAR_PENDING3_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPENDSGIR1")
            .field("sgi_clear_pending0", &self.sgi_clear_pending0())
            .field("sgi_clear_pending1", &self.sgi_clear_pending1())
            .field("sgi_clear_pending2", &self.sgi_clear_pending2())
            .field("sgi_clear_pending3", &self.sgi_clear_pending3())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SGI_CLEAR_PENDING0
    #[inline(always)]
    pub fn sgi_clear_pending0(&mut self) -> SGI_CLEAR_PENDING0_W<'_, CPENDSGIR1rs> {
        SGI_CLEAR_PENDING0_W::new(self, 0)
    }
    ///Bits 8:9 - SGI_CLEAR_PENDING1
    #[inline(always)]
    pub fn sgi_clear_pending1(&mut self) -> SGI_CLEAR_PENDING1_W<'_, CPENDSGIR1rs> {
        SGI_CLEAR_PENDING1_W::new(self, 8)
    }
    ///Bits 16:17 - SGI_CLEAR_PENDING2
    #[inline(always)]
    pub fn sgi_clear_pending2(&mut self) -> SGI_CLEAR_PENDING2_W<'_, CPENDSGIR1rs> {
        SGI_CLEAR_PENDING2_W::new(self, 16)
    }
    ///Bits 24:25 - SGI_CLEAR_PENDING3
    #[inline(always)]
    pub fn sgi_clear_pending3(&mut self) -> SGI_CLEAR_PENDING3_W<'_, CPENDSGIR1rs> {
        SGI_CLEAR_PENDING3_W::new(self, 24)
    }
}
/**For SGI x*4 to SGI x*4+3

You can [`read`](crate::Reg::read) this register and get [`cpendsgir1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpendsgir1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:CPENDSGIR1)*/
pub struct CPENDSGIR1rs;
impl crate::RegisterSpec for CPENDSGIR1rs {
    type Ux = u32;
}
///`read()` method returns [`cpendsgir1::R`](R) reader structure
impl crate::Readable for CPENDSGIR1rs {}
///`write(|w| ..)` method takes [`cpendsgir1::W`](W) writer structure
impl crate::Writable for CPENDSGIR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CPENDSGIR1 to value 0
impl crate::Resettable for CPENDSGIR1rs {}
