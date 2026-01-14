///Register `AXIDIVR` reader
pub type R = crate::R<AXIDIVRrs>;
///Register `AXIDIVR` writer
pub type W = crate::W<AXIDIVRrs>;
///Field `AXIDIV` reader - AXIDIV
pub type AXIDIV_R = crate::FieldReader;
///Field `AXIDIV` writer - AXIDIV
pub type AXIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AXIDIVRDY` reader - AXIDIVRDY
pub type AXIDIVRDY_R = crate::BitReader;
impl R {
    ///Bits 0:2 - AXIDIV
    #[inline(always)]
    pub fn axidiv(&self) -> AXIDIV_R {
        AXIDIV_R::new((self.bits & 7) as u8)
    }
    ///Bit 31 - AXIDIVRDY
    #[inline(always)]
    pub fn axidivrdy(&self) -> AXIDIVRDY_R {
        AXIDIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXIDIVR")
            .field("axidiv", &self.axidiv())
            .field("axidivrdy", &self.axidivrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - AXIDIV
    #[inline(always)]
    pub fn axidiv(&mut self) -> AXIDIV_W<'_, AXIDIVRrs> {
        AXIDIV_W::new(self, 0)
    }
}
/**This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`axidivr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axidivr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:AXIDIVR)*/
pub struct AXIDIVRrs;
impl crate::RegisterSpec for AXIDIVRrs {
    type Ux = u32;
}
///`read()` method returns [`axidivr::R`](R) reader structure
impl crate::Readable for AXIDIVRrs {}
///`write(|w| ..)` method takes [`axidivr::W`](W) writer structure
impl crate::Writable for AXIDIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AXIDIVR to value 0x8000_0000
impl crate::Resettable for AXIDIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
