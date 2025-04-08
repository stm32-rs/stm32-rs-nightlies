///Register `DR_01` reader
pub type R = crate::R<DR_01rs>;
///Field `PE` reader - Parity Error bit
pub type PE_R = crate::BitReader;
///Field `V` reader - Validity bit
pub type V_R = crate::BitReader;
///Field `U` reader - User bit
pub type U_R = crate::BitReader;
///Field `C` reader - Channel Status bit
pub type C_R = crate::BitReader;
///Field `PT` reader - Preamble Type
pub type PT_R = crate::FieldReader;
///Field `DR` reader - Data value
pub type DR_R = crate::FieldReader<u32>;
impl R {
    ///Bit 0 - Parity Error bit
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Validity bit
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - User bit
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel Status bit
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Preamble Type
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:31 - Data value
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR_01")
            .field("pe", &self.pe())
            .field("v", &self.v())
            .field("u", &self.u())
            .field("c", &self.c())
            .field("pt", &self.pt())
            .field("dr", &self.dr())
            .finish()
    }
}
/**Data input register

You can [`read`](crate::Reg::read) this register and get [`dr_01::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#SPDIFRX:DR_01)*/
pub struct DR_01rs;
impl crate::RegisterSpec for DR_01rs {
    type Ux = u32;
}
///`read()` method returns [`dr_01::R`](R) reader structure
impl crate::Readable for DR_01rs {}
///`reset()` method sets DR_01 to value 0
impl crate::Resettable for DR_01rs {}
