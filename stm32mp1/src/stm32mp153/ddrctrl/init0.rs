///Register `INIT0` reader
pub type R = crate::R<INIT0rs>;
///Register `INIT0` writer
pub type W = crate::W<INIT0rs>;
///Field `PRE_CKE_X1024` reader - PRE_CKE_X1024
pub type PRE_CKE_X1024_R = crate::FieldReader<u16>;
///Field `PRE_CKE_X1024` writer - PRE_CKE_X1024
pub type PRE_CKE_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `POST_CKE_X1024` reader - POST_CKE_X1024
pub type POST_CKE_X1024_R = crate::FieldReader<u16>;
///Field `POST_CKE_X1024` writer - POST_CKE_X1024
pub type POST_CKE_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `SKIP_DRAM_INIT` reader - SKIP_DRAM_INIT
pub type SKIP_DRAM_INIT_R = crate::FieldReader;
///Field `SKIP_DRAM_INIT` writer - SKIP_DRAM_INIT
pub type SKIP_DRAM_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:11 - PRE_CKE_X1024
    #[inline(always)]
    pub fn pre_cke_x1024(&self) -> PRE_CKE_X1024_R {
        PRE_CKE_X1024_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:25 - POST_CKE_X1024
    #[inline(always)]
    pub fn post_cke_x1024(&self) -> POST_CKE_X1024_R {
        POST_CKE_X1024_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 30:31 - SKIP_DRAM_INIT
    #[inline(always)]
    pub fn skip_dram_init(&self) -> SKIP_DRAM_INIT_R {
        SKIP_DRAM_INIT_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INIT0")
            .field("pre_cke_x1024", &self.pre_cke_x1024())
            .field("post_cke_x1024", &self.post_cke_x1024())
            .field("skip_dram_init", &self.skip_dram_init())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - PRE_CKE_X1024
    #[inline(always)]
    pub fn pre_cke_x1024(&mut self) -> PRE_CKE_X1024_W<'_, INIT0rs> {
        PRE_CKE_X1024_W::new(self, 0)
    }
    ///Bits 16:25 - POST_CKE_X1024
    #[inline(always)]
    pub fn post_cke_x1024(&mut self) -> POST_CKE_X1024_W<'_, INIT0rs> {
        POST_CKE_X1024_W::new(self, 16)
    }
    ///Bits 30:31 - SKIP_DRAM_INIT
    #[inline(always)]
    pub fn skip_dram_init(&mut self) -> SKIP_DRAM_INIT_W<'_, INIT0rs> {
        SKIP_DRAM_INIT_W::new(self, 30)
    }
}
/**DDRCTRL SDRAM initialization register 0

You can [`read`](crate::Reg::read) this register and get [`init0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:INIT0)*/
pub struct INIT0rs;
impl crate::RegisterSpec for INIT0rs {
    type Ux = u32;
}
///`read()` method returns [`init0::R`](R) reader structure
impl crate::Readable for INIT0rs {}
///`write(|w| ..)` method takes [`init0::W`](W) writer structure
impl crate::Writable for INIT0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INIT0 to value 0x0002_004e
impl crate::Resettable for INIT0rs {
    const RESET_VALUE: u32 = 0x0002_004e;
}
