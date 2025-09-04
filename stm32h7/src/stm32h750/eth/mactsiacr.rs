///Register `MACTSIACR` reader
pub type R = crate::R<MACTSIACRrs>;
///Register `MACTSIACR` writer
pub type W = crate::W<MACTSIACRrs>;
///Field `OSTIAC` reader - One-Step Timestamp Ingress Asymmetry Correction This field contains the ingress path asymmetry value to be added to correctionField of Pdelay_Resp PTP packet. The programmed value should be in units of nanoseconds and multiplied by 2^16. For example, 2.5 ns is represented as 0x00028000. The value can also be negative, which is represented in 2's complement form with bit 31 representing the sign bit.
pub type OSTIAC_R = crate::FieldReader<u32>;
///Field `OSTIAC` writer - One-Step Timestamp Ingress Asymmetry Correction This field contains the ingress path asymmetry value to be added to correctionField of Pdelay_Resp PTP packet. The programmed value should be in units of nanoseconds and multiplied by 2^16. For example, 2.5 ns is represented as 0x00028000. The value can also be negative, which is represented in 2's complement form with bit 31 representing the sign bit.
pub type OSTIAC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - One-Step Timestamp Ingress Asymmetry Correction This field contains the ingress path asymmetry value to be added to correctionField of Pdelay_Resp PTP packet. The programmed value should be in units of nanoseconds and multiplied by 2^16. For example, 2.5 ns is represented as 0x00028000. The value can also be negative, which is represented in 2's complement form with bit 31 representing the sign bit.
    #[inline(always)]
    pub fn ostiac(&self) -> OSTIAC_R {
        OSTIAC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSIACR")
            .field("ostiac", &self.ostiac())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - One-Step Timestamp Ingress Asymmetry Correction This field contains the ingress path asymmetry value to be added to correctionField of Pdelay_Resp PTP packet. The programmed value should be in units of nanoseconds and multiplied by 2^16. For example, 2.5 ns is represented as 0x00028000. The value can also be negative, which is represented in 2's complement form with bit 31 representing the sign bit.
    #[inline(always)]
    pub fn ostiac(&mut self) -> OSTIAC_W<MACTSIACRrs> {
        OSTIAC_W::new(self, 0)
    }
}
/**Timestamp Ingress asymmetric correction register

You can [`read`](crate::Reg::read) this register and get [`mactsiacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsiacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:MACTSIACR)*/
pub struct MACTSIACRrs;
impl crate::RegisterSpec for MACTSIACRrs {
    type Ux = u32;
}
///`read()` method returns [`mactsiacr::R`](R) reader structure
impl crate::Readable for MACTSIACRrs {}
///`write(|w| ..)` method takes [`mactsiacr::W`](W) writer structure
impl crate::Writable for MACTSIACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACTSIACR to value 0
impl crate::Resettable for MACTSIACRrs {}
