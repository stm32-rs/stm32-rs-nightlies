///Register `P1CST1CR` reader
pub type R = crate::R<P1CST1CRrs>;
///Field `ENABLE` reader - Current enable bit value
pub type ENABLE_R = crate::BitReader;
///Field `BINS` reader - Current bin definition
pub type BINS_R = crate::FieldReader;
///Field `SRC` reader - Current source of statistics
pub type SRC_R = crate::FieldReader;
///Field `MODE` reader - Current statistics mode
pub type MODE_R = crate::BitReader;
///Field `ACCU` reader - Current accumulation result, divided by 256.
pub type ACCU_R = crate::FieldReader<u32>;
impl R {
    ///Bit 0 - Current enable bit value
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Current bin definition
    #[inline(always)]
    pub fn bins(&self) -> BINS_R {
        BINS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Current source of statistics
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Current statistics mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:31 - Current accumulation result, divided by 256.
    #[inline(always)]
    pub fn accu(&self) -> ACCU_R {
        ACCU_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CST1CR")
            .field("enable", &self.enable())
            .field("bins", &self.bins())
            .field("src", &self.src())
            .field("mode", &self.mode())
            .field("accu", &self.accu())
            .finish()
    }
}
/**DCMIPP Pipe1 current statistics 1 control register

You can [`read`](crate::Reg::read) this register and get [`p1cst1cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1CST1CR)*/
pub struct P1CST1CRrs;
impl crate::RegisterSpec for P1CST1CRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cst1cr::R`](R) reader structure
impl crate::Readable for P1CST1CRrs {}
///`reset()` method sets P1CST1CR to value 0
impl crate::Resettable for P1CST1CRrs {}
