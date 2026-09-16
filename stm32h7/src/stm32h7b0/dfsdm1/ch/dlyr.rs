///Register `DLYR` reader
pub type R = crate::R<DLYRrs>;
///Register `DLYR` writer
pub type W = crate::W<DLYRrs>;
///Field `PLSSKP` reader - Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\[5:0\] returns current value of pulses which will be skipped. If PLSSKP\[5:0\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\[5:0\] also when PLSSKP\[5:0\] is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied
pub type PLSSKP_R = crate::FieldReader;
///Field `PLSSKP` writer - Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\[5:0\] returns current value of pulses which will be skipped. If PLSSKP\[5:0\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\[5:0\] also when PLSSKP\[5:0\] is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied
pub type PLSSKP_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    ///Bits 0:5 - Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\[5:0\] returns current value of pulses which will be skipped. If PLSSKP\[5:0\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\[5:0\] also when PLSSKP\[5:0\] is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLYR")
            .field("plsskp", &self.plsskp())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\[5:0\] returns current value of pulses which will be skipped. If PLSSKP\[5:0\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\[5:0\] also when PLSSKP\[5:0\] is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied
    #[inline(always)]
    pub fn plsskp(&mut self) -> PLSSKP_W<'_, DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DLYRrs;
impl crate::RegisterSpec for DLYRrs {
    type Ux = u32;
}
///`read()` method returns [`dlyr::R`](R) reader structure
impl crate::Readable for DLYRrs {}
///`write(|w| ..)` method takes [`dlyr::W`](W) writer structure
impl crate::Writable for DLYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLYR to value 0
impl crate::Resettable for DLYRrs {}
