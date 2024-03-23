#[doc = "Register `MACTSEACR` reader"]
pub type R = crate::R<MACTSEACRrs>;
#[doc = "Register `MACTSEACR` writer"]
pub type W = crate::W<MACTSEACRrs>;
#[doc = "Field `OSTEAC` reader - One-Step Timestamp Egress Asymmetry Correction This field contains the egress path asymmetry value to be subtracted from correctionField of Pdelay_Resp PTP packet. The programmed value must be the negated value in units of nanoseconds multiplied by 2^16. For example, if the required correction is +2.5 ns, the programmed value must be 0xFFFD_8000, which is the 2's complement of 0x0002_8000(2.5 * 2^16). Similarly, if the required correction is -3.3 ns, the programmed value is 0x0003_4CCC (3.3 *2^16)."]
pub type OSTEAC_R = crate::FieldReader<u32>;
#[doc = "Field `OSTEAC` writer - One-Step Timestamp Egress Asymmetry Correction This field contains the egress path asymmetry value to be subtracted from correctionField of Pdelay_Resp PTP packet. The programmed value must be the negated value in units of nanoseconds multiplied by 2^16. For example, if the required correction is +2.5 ns, the programmed value must be 0xFFFD_8000, which is the 2's complement of 0x0002_8000(2.5 * 2^16). Similarly, if the required correction is -3.3 ns, the programmed value is 0x0003_4CCC (3.3 *2^16)."]
pub type OSTEAC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction This field contains the egress path asymmetry value to be subtracted from correctionField of Pdelay_Resp PTP packet. The programmed value must be the negated value in units of nanoseconds multiplied by 2^16. For example, if the required correction is +2.5 ns, the programmed value must be 0xFFFD_8000, which is the 2's complement of 0x0002_8000(2.5 * 2^16). Similarly, if the required correction is -3.3 ns, the programmed value is 0x0003_4CCC (3.3 *2^16)."]
    #[inline(always)]
    pub fn osteac(&self) -> OSTEAC_R {
        OSTEAC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - One-Step Timestamp Egress Asymmetry Correction This field contains the egress path asymmetry value to be subtracted from correctionField of Pdelay_Resp PTP packet. The programmed value must be the negated value in units of nanoseconds multiplied by 2^16. For example, if the required correction is +2.5 ns, the programmed value must be 0xFFFD_8000, which is the 2's complement of 0x0002_8000(2.5 * 2^16). Similarly, if the required correction is -3.3 ns, the programmed value is 0x0003_4CCC (3.3 *2^16)."]
    #[inline(always)]
    #[must_use]
    pub fn osteac(&mut self) -> OSTEAC_W<MACTSEACRrs> {
        OSTEAC_W::new(self, 0)
    }
}
#[doc = "Timestamp Egress asymmetric correction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactseacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactseacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSEACRrs;
impl crate::RegisterSpec for MACTSEACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactseacr::R`](R) reader structure"]
impl crate::Readable for MACTSEACRrs {}
#[doc = "`write(|w| ..)` method takes [`mactseacr::W`](W) writer structure"]
impl crate::Writable for MACTSEACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTSEACR to value 0"]
impl crate::Resettable for MACTSEACRrs {
    const RESET_VALUE: u32 = 0;
}
