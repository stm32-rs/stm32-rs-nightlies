#[doc = "Register `MACPPSIR` reader"]
pub type R = crate::R<MACPPSIRrs>;
#[doc = "Register `MACPPSIR` writer"]
pub type W = crate::W<MACPPSIRrs>;
#[doc = "Field `PPSINT0` reader - PPS Output Signal Interval These bits store the interval between the rising edges of PPS signal output. The interval is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20 ns), and desired interval between the rising edges of PPS signal output is 100 ns (that is, 5 units of subsecond increment value), you should program value 4 (5-1) in this register."]
pub type PPSINT0_R = crate::FieldReader<u32>;
#[doc = "Field `PPSINT0` writer - PPS Output Signal Interval These bits store the interval between the rising edges of PPS signal output. The interval is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20 ns), and desired interval between the rising edges of PPS signal output is 100 ns (that is, 5 units of subsecond increment value), you should program value 4 (5-1) in this register."]
pub type PPSINT0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPS Output Signal Interval These bits store the interval between the rising edges of PPS signal output. The interval is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20 ns), and desired interval between the rising edges of PPS signal output is 100 ns (that is, 5 units of subsecond increment value), you should program value 4 (5-1) in this register."]
    #[inline(always)]
    pub fn ppsint0(&self) -> PPSINT0_R {
        PPSINT0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS Output Signal Interval These bits store the interval between the rising edges of PPS signal output. The interval is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20 ns), and desired interval between the rising edges of PPS signal output is 100 ns (that is, 5 units of subsecond increment value), you should program value 4 (5-1) in this register."]
    #[inline(always)]
    #[must_use]
    pub fn ppsint0(&mut self) -> PPSINT0_W<MACPPSIRrs> {
        PPSINT0_W::new(self, 0)
    }
}
#[doc = "PPS interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppsir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppsir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPPSIRrs;
impl crate::RegisterSpec for MACPPSIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppsir::R`](R) reader structure"]
impl crate::Readable for MACPPSIRrs {}
#[doc = "`write(|w| ..)` method takes [`macppsir::W`](W) writer structure"]
impl crate::Writable for MACPPSIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPPSIR to value 0"]
impl crate::Resettable for MACPPSIRrs {
    const RESET_VALUE: u32 = 0;
}
