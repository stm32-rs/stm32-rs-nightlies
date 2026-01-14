///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `LCAB_DAMP_THRES` reader - LCAB_DAMP_THRES\[7:0\]: Damping threshold for LCA and LCB
pub type LCAB_DAMP_THRES_R = crate::FieldReader;
///Field `LCAB_DAMP_THRES` writer - LCAB_DAMP_THRES\[7:0\]: Damping threshold for LCA and LCB
pub type LCAB_DAMP_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TREC_VCM` reader - VCMBUFF Recovery Time
pub type TREC_VCM_R = crate::FieldReader<u16>;
///Field `TREC_VCM` writer - VCMBUFF Recovery Time
pub type TREC_VCM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `TSTART_VCM` reader - VCMBUFF Starting Time
pub type TSTART_VCM_R = crate::FieldReader<u16>;
///Field `TSTART_VCM` writer - VCMBUFF Starting Time
pub type TSTART_VCM_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:7 - LCAB_DAMP_THRES\[7:0\]: Damping threshold for LCA and LCB
    #[inline(always)]
    pub fn lcab_damp_thres(&self) -> LCAB_DAMP_THRES_R {
        LCAB_DAMP_THRES_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 10:18 - VCMBUFF Recovery Time
    #[inline(always)]
    pub fn trec_vcm(&self) -> TREC_VCM_R {
        TREC_VCM_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    ///Bits 20:30 - VCMBUFF Starting Time
    #[inline(always)]
    pub fn tstart_vcm(&self) -> TSTART_VCM_R {
        TSTART_VCM_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lcab_damp_thres", &self.lcab_damp_thres())
            .field("trec_vcm", &self.trec_vcm())
            .field("tstart_vcm", &self.tstart_vcm())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - LCAB_DAMP_THRES\[7:0\]: Damping threshold for LCA and LCB
    #[inline(always)]
    pub fn lcab_damp_thres(&mut self) -> LCAB_DAMP_THRES_W<'_, CR1rs> {
        LCAB_DAMP_THRES_W::new(self, 0)
    }
    ///Bits 10:18 - VCMBUFF Recovery Time
    #[inline(always)]
    pub fn trec_vcm(&mut self) -> TREC_VCM_W<'_, CR1rs> {
        TREC_VCM_W::new(self, 10)
    }
    ///Bits 20:30 - VCMBUFF Starting Time
    #[inline(always)]
    pub fn tstart_vcm(&mut self) -> TSTART_VCM_W<'_, CR1rs> {
        TSTART_VCM_W::new(self, 20)
    }
}
/**LCSC_CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0x3c01_0c80
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0x3c01_0c80;
}
