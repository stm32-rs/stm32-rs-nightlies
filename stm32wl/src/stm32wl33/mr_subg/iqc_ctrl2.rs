///Register `IQC_CTRL2` reader
pub type R = crate::R<IQC_CTRL2rs>;
///Register `IQC_CTRL2` writer
pub type W = crate::W<IQC_CTRL2rs>;
///Field `QPD_DECAY` reader - Decay coefficient for QPD:
pub type QPD_DECAY_R = crate::FieldReader;
///Field `QPD_DECAY` writer - Decay coefficient for QPD:
pub type QPD_DECAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Decay coefficient for QPD:
    #[inline(always)]
    pub fn qpd_decay(&self) -> QPD_DECAY_R {
        QPD_DECAY_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IQC_CTRL2")
            .field("qpd_decay", &self.qpd_decay())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Decay coefficient for QPD:
    #[inline(always)]
    pub fn qpd_decay(&mut self) -> QPD_DECAY_W<'_, IQC_CTRL2rs> {
        QPD_DECAY_W::new(self, 0)
    }
}
/**IQC_CTRL2 register

You can [`read`](crate::Reg::read) this register and get [`iqc_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:IQC_CTRL2)*/
pub struct IQC_CTRL2rs;
impl crate::RegisterSpec for IQC_CTRL2rs {
    type Ux = u32;
}
///`read()` method returns [`iqc_ctrl2::R`](R) reader structure
impl crate::Readable for IQC_CTRL2rs {}
///`write(|w| ..)` method takes [`iqc_ctrl2::W`](W) writer structure
impl crate::Writable for IQC_CTRL2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IQC_CTRL2 to value 0x08
impl crate::Resettable for IQC_CTRL2rs {
    const RESET_VALUE: u32 = 0x08;
}
