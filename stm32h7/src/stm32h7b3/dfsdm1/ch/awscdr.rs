///Register `AWSCDR` reader
pub type R = crate::R<AWSCDRrs>;
///Register `AWSCDR` writer
pub type W = crate::W<AWSCDRrs>;
///Field `SCDT` reader - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel.
pub type SCDT_R = crate::FieldReader;
///Field `SCDT` writer - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel.
pub type SCDT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BKSCD` reader - Break signal assignment for short-circuit detector on channel y BKSCD\[i\] = 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\[i\] = 1: Break i signal assigned to short-circuit detector on channel y
pub type BKSCD_R = crate::FieldReader;
///Field `BKSCD` writer - Break signal assignment for short-circuit detector on channel y BKSCD\[i\] = 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\[i\] = 1: Break i signal assigned to short-circuit detector on channel y
pub type BKSCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AWFOSR` reader - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is
pub type AWFOSR_R = crate::FieldReader;
///Field `AWFOSR` writer - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is
pub type AWFOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `AWFORD` reader - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type AWFORD_R = crate::FieldReader;
///Field `AWFORD` writer - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
pub type AWFORD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel.
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 12:15 - Break signal assignment for short-circuit detector on channel y BKSCD\[i\] = 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\[i\] = 1: Break i signal assigned to short-circuit detector on channel y
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 22:23 - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWSCDR")
            .field("scdt", &self.scdt())
            .field("bkscd", &self.bkscd())
            .field("awfosr", &self.awfosr())
            .field("awford", &self.awford())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel.
    #[inline(always)]
    pub fn scdt(&mut self) -> SCDT_W<AWSCDRrs> {
        SCDT_W::new(self, 0)
    }
    ///Bits 12:15 - Break signal assignment for short-circuit detector on channel y BKSCD\[i\] = 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\[i\] = 1: Break i signal assigned to short-circuit detector on channel y
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W<AWSCDRrs> {
        BKSCD_W::new(self, 12)
    }
    ///Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1). This number is
    #[inline(always)]
    pub fn awfosr(&mut self) -> AWFOSR_W<AWSCDRrs> {
        AWFOSR_W::new(self, 16)
    }
    ///Bits 22:23 - Analog watchdog Sinc filter order on channel y 2: Sinc2 filter type 3: Sinc3 filter type Sincx filter type transfer function: FastSinc filter type transfer function: This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register).
    #[inline(always)]
    pub fn awford(&mut self) -> AWFORD_W<AWSCDRrs> {
        AWFORD_W::new(self, 22)
    }
}
/**DFSDM channel 0 analog watchdog and short-circuit detector register

You can [`read`](crate::Reg::read) this register and get [`awscdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awscdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AWSCDRrs;
impl crate::RegisterSpec for AWSCDRrs {
    type Ux = u32;
}
///`read()` method returns [`awscdr::R`](R) reader structure
impl crate::Readable for AWSCDRrs {}
///`write(|w| ..)` method takes [`awscdr::W`](W) writer structure
impl crate::Writable for AWSCDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWSCDR to value 0
impl crate::Resettable for AWSCDRrs {}
