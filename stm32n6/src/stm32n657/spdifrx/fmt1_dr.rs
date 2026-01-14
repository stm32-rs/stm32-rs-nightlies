///Register `FMT1_DR` reader
pub type R = crate::R<FMT1_DRrs>;
///Field `PE` reader - parity error bit
pub type PE_R = crate::BitReader;
///Field `V` reader - validity bit
pub type V_R = crate::BitReader;
///Field `U` reader - user bit
pub type U_R = crate::BitReader;
///Field `C` reader - channel Status bit
pub type C_R = crate::BitReader;
///Field `PT` reader - preamble type
pub type PT_R = crate::FieldReader;
///Field `DR` reader - data value
pub type DR_R = crate::FieldReader<u32>;
impl R {
    ///Bit 0 - parity error bit
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - validity bit
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - user bit
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - channel Status bit
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - preamble type
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:31 - data value
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMT1_DR")
            .field("pe", &self.pe())
            .field("v", &self.v())
            .field("u", &self.u())
            .field("c", &self.c())
            .field("pt", &self.pt())
            .field("dr", &self.dr())
            .finish()
    }
}
/**SPDIFRX data input register

You can [`read`](crate::Reg::read) this register and get [`fmt1_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SPDIFRX:FMT1_DR)*/
pub struct FMT1_DRrs;
impl crate::RegisterSpec for FMT1_DRrs {
    type Ux = u32;
}
///`read()` method returns [`fmt1_dr::R`](R) reader structure
impl crate::Readable for FMT1_DRrs {}
///`reset()` method sets FMT1_DR to value 0
impl crate::Resettable for FMT1_DRrs {}
