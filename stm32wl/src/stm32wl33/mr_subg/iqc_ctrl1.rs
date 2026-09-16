///Register `IQC_CTRL1` reader
pub type R = crate::R<IQC_CTRL1rs>;
///Register `IQC_CTRL1` writer
pub type W = crate::W<IQC_CTRL1rs>;
///Field `QPD_ATTACK` reader - Attack coefficient for QPD:
pub type QPD_ATTACK_R = crate::FieldReader;
///Field `QPD_ATTACK` writer - Attack coefficient for QPD:
pub type QPD_ATTACK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Attack coefficient for QPD:
    #[inline(always)]
    pub fn qpd_attack(&self) -> QPD_ATTACK_R {
        QPD_ATTACK_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IQC_CTRL1")
            .field("qpd_attack", &self.qpd_attack())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Attack coefficient for QPD:
    #[inline(always)]
    pub fn qpd_attack(&mut self) -> QPD_ATTACK_W<'_, IQC_CTRL1rs> {
        QPD_ATTACK_W::new(self, 0)
    }
}
/**IQC_CTRL1 register

You can [`read`](crate::Reg::read) this register and get [`iqc_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:IQC_CTRL1)*/
pub struct IQC_CTRL1rs;
impl crate::RegisterSpec for IQC_CTRL1rs {
    type Ux = u32;
}
///`read()` method returns [`iqc_ctrl1::R`](R) reader structure
impl crate::Readable for IQC_CTRL1rs {}
///`write(|w| ..)` method takes [`iqc_ctrl1::W`](W) writer structure
impl crate::Writable for IQC_CTRL1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IQC_CTRL1 to value 0x08
impl crate::Resettable for IQC_CTRL1rs {
    const RESET_VALUE: u32 = 0x08;
}
