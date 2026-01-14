///Register `FMT0_DR` reader
pub type R = crate::R<FMT0_DRrs>;
///Field `DR` reader - data value
pub type DR_R = crate::FieldReader<u32>;
///Field `PE` reader - parity error bit
pub type PE_R = crate::BitReader;
///Field `V` reader - validity bit
pub type V_R = crate::BitReader;
///Field `U` reader - user bit
pub type U_R = crate::BitReader;
///Field `C` reader - channel status bit
pub type C_R = crate::BitReader;
///Field `PT` reader - preamble type
pub type PT_R = crate::FieldReader;
impl R {
    ///Bits 0:23 - data value
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - parity error bit
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - validity bit
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - user bit
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - channel status bit
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - preamble type
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMT0_DR")
            .field("dr", &self.dr())
            .field("pe", &self.pe())
            .field("v", &self.v())
            .field("u", &self.u())
            .field("c", &self.c())
            .field("pt", &self.pt())
            .finish()
    }
}
/**SPDIFRX data input register

You can [`read`](crate::Reg::read) this register and get [`fmt0_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SPDIFRX:FMT0_DR)*/
pub struct FMT0_DRrs;
impl crate::RegisterSpec for FMT0_DRrs {
    type Ux = u32;
}
///`read()` method returns [`fmt0_dr::R`](R) reader structure
impl crate::Readable for FMT0_DRrs {}
///`reset()` method sets FMT0_DR to value 0
impl crate::Resettable for FMT0_DRrs {}
