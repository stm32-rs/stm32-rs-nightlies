///Register `MDF_DLY1CR` reader
pub type R = crate::R<MDF_DLY1CRrs>;
///Register `MDF_DLY1CR` writer
pub type W = crate::W<MDF_DLY1CRrs>;
///Field `SKPDLY` reader - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,
pub type SKPDLY_R = crate::FieldReader;
///Field `SKPDLY` writer - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,
pub type SKPDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Field `SKPBF` reader - Skip Busy flag Set and cleared by hardware. Shall be used in order to control if the delay sequence is completed. - 0: Reading 0 means that the MDF is ready to accept a new value into SKPDLY\[6:0\]. - 1: Reading 1 means that last valid SKPDLY\[6:0\]
is still under precessing.*/
pub type SKPBF_R = crate::BitReader;
impl R {
    ///Bits 0:6 - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,
    #[inline(always)]
    pub fn skpdly(&self) -> SKPDLY_R {
        SKPDLY_R::new((self.bits & 0x7f) as u8)
    }
    /**Bit 31 - Skip Busy flag Set and cleared by hardware. Shall be used in order to control if the delay sequence is completed. - 0: Reading 0 means that the MDF is ready to accept a new value into SKPDLY\[6:0\]. - 1: Reading 1 means that last valid SKPDLY\[6:0\]
    is still under precessing.*/
    #[inline(always)]
    pub fn skpbf(&self) -> SKPBF_R {
        SKPBF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDF_DLY1CR")
            .field("skpdly", &self.skpdly())
            .field("skpbf", &self.skpbf())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Delay to apply to a bitstream Set and cleared by software. Defines the number of input samples that will be skipped. Skipping is applied immediately after writing to this field, if SKPBF = 0 , and the corresponding bit DFLTEN = 1 . If SKPBF = 1 the value written into the register is ignored by the delay state machine. - 0: No input sample skipped, - 1: 1 input sample skipped, ... - 127: 127 input sample skipped,
    #[inline(always)]
    pub fn skpdly(&mut self) -> SKPDLY_W<MDF_DLY1CRrs> {
        SKPDLY_W::new(self, 0)
    }
}
/**This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_dly1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dly1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#MDF1:MDF_DLY1CR)*/
pub struct MDF_DLY1CRrs;
impl crate::RegisterSpec for MDF_DLY1CRrs {
    type Ux = u32;
}
///`read()` method returns [`mdf_dly1cr::R`](R) reader structure
impl crate::Readable for MDF_DLY1CRrs {}
///`write(|w| ..)` method takes [`mdf_dly1cr::W`](W) writer structure
impl crate::Writable for MDF_DLY1CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDF_DLY1CR to value 0
impl crate::Resettable for MDF_DLY1CRrs {
    const RESET_VALUE: u32 = 0;
}
