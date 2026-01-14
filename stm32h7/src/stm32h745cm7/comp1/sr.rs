///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `C1VAL` reader - COMP channel 1 output status bit
pub type C1VAL_R = crate::BitReader;
///Field `C2VAL` reader - COMP channel 2 output status bit
pub type C2VAL_R = crate::BitReader;
///Field `C1IF` reader - COMP channel 1 Interrupt Flag
pub type C1IF_R = crate::BitReader;
///Field `C2IF` reader - COMP channel 2 Interrupt Flag
pub type C2IF_R = crate::BitReader;
impl R {
    ///Bit 0 - COMP channel 1 output status bit
    #[inline(always)]
    pub fn c1val(&self) -> C1VAL_R {
        C1VAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - COMP channel 2 output status bit
    #[inline(always)]
    pub fn c2val(&self) -> C2VAL_R {
        C2VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - COMP channel 1 Interrupt Flag
    #[inline(always)]
    pub fn c1if(&self) -> C1IF_R {
        C1IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - COMP channel 2 Interrupt Flag
    #[inline(always)]
    pub fn c2if(&self) -> C2IF_R {
        C2IF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("c1val", &self.c1val())
            .field("c2val", &self.c2val())
            .field("c1if", &self.c1if())
            .field("c2if", &self.c2if())
            .finish()
    }
}
/**Comparator status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#COMP1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
