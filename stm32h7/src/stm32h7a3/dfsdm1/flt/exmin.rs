///Register `EXMIN` reader
pub type R = crate::R<EXMINrs>;
///Register `EXMIN` writer
pub type W = crate::W<EXMINrs>;
///Field `EXMINCH` reader - Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN\[23:0\]. Bits are cleared by reading of this register.
pub type EXMINCH_R = crate::FieldReader;
/**Field `EXMIN` reader - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\[23:0\] bits are reset to value (0x7FFFFF) by reading of this register.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type EXMIN_R = crate::FieldReader<u32>;
///Field `EXMIN` writer - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\[23:0\] bits are reset to value (0x7FFFFF) by reading of this register.
pub type EXMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32, crate::Safe>;
impl R {
    ///Bits 0:2 - Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN\[23:0\]. Bits are cleared by reading of this register.
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\[23:0\] bits are reset to value (0x7FFFFF) by reading of this register.
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXMIN")
            .field("exminch", &self.exminch())
            .finish()
    }
}
impl W {
    ///Bits 8:31 - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\[23:0\] bits are reset to value (0x7FFFFF) by reading of this register.
    #[inline(always)]
    pub fn exmin(&mut self) -> EXMIN_W<'_, EXMINrs> {
        EXMIN_W::new(self, 8)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`exmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXMINrs;
impl crate::RegisterSpec for EXMINrs {
    type Ux = u32;
}
///`read()` method returns [`exmin::R`](R) reader structure
impl crate::Readable for EXMINrs {}
///`write(|w| ..)` method takes [`exmin::W`](W) writer structure
impl crate::Writable for EXMINrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXMIN to value 0x7fff_ff00
impl crate::Resettable for EXMINrs {
    const RESET_VALUE: u32 = 0x7fff_ff00;
}
